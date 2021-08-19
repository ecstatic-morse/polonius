use std::collections::HashMap;
use std::convert::TryInto;
use std::pin::Pin;

use cxx::let_cxx_string;
use log::warn;
use polonius_engine::{AllFacts, FactTypes};

use crate::ffi::{self, IntoTuple};

macro_rules! load_facts {
    ($prog:ident, $facts:ident; $( $f:ident ),* $(,)?) => {
        // Exhaustive matching, since new facts must be reflected below as well.
        let AllFacts {
            $( $f ),*
        } = $facts;

        // FIXME: Extract this into a function outside the macro.
        $(
            let_cxx_string!(rel_name = stringify!($f));
            let rel = $prog.as_mut().getRelation(&rel_name);

            // SAFETY: `getRelation` returns either a valid pointer or null.
            let rel = unsafe { rel.as_mut() };

            if let Some(rel) = rel {
                // SAFETY: TODO
                let mut rel = unsafe { Pin::new_unchecked(rel) };

                let arity: usize = rel.getArity().try_into().unwrap();

                for &t in $f {
                    // Assert that the relation has the correct arity.
                    //
                    // FIXME: Is there a better way to do this? Preferably outside the loop?
                    debug_assert_eq!(std::mem::size_of_val(&t) % std::mem::size_of::<u32>(), 0);
                    let datafrog_arity = std::mem::size_of_val(&t) / std::mem::size_of::<u32>();
                    if arity != datafrog_arity {
                        panic!(r#"Arity mismatch for "{}". souffle={}, datafrog={}"#, stringify!($f), arity, datafrog_arity);
                    }

                    t.into_tuple().insert_into_relation($prog.as_mut(), rel.as_mut())
                }
            } else {
                warn!("Relation named `{}` not found. Skipping...", stringify!($f));
            }
        )*
    }
}

pub fn insert_all_facts<T>(mut prog: Pin<&mut ffi::Program>, facts: &AllFacts<T>)
where
    T: FactTypes,
    T::Origin: Into<u32>,
    T::Loan: Into<u32>,
    T::Point: Into<u32>,
    T::Variable: Into<u32>,
    T::Path: Into<u32>,
{
    load_facts!(prog, facts;
        loan_issued_at,
        universal_region,
        cfg_edge,
        loan_killed_at,
        subset_base,
        loan_invalidated_at,
        var_used_at,
        var_defined_at,
        var_dropped_at,
        use_of_var_derefs_origin,
        drop_of_var_derefs_origin,
        child_path,
        path_is_var,
        path_assigned_at_base,
        path_moved_at_base,
        path_accessed_at_base,
        known_placeholder_subset,
        placeholder,
    );
}

pub fn extract_output_facts(prog: Pin<&mut ffi::Program>) -> HashMap<String, ffi::DynTuples> {
    prog.get_output_relations()
        .map(|rel| {
            let s = ffi::get_name(rel).to_str().unwrap().to_owned();
            let tuples = ffi::dump_tuples(rel);
            (s, tuples)
        })
        .collect()
}
