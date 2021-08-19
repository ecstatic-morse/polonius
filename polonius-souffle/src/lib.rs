pub mod facts;
mod ffi;

pub use ffi::DynTuples;

use std::collections::HashMap;
use std::path::Path;

use cxx::let_cxx_string;
use polonius_engine::{AllFacts, FactTypes};

pub fn run_from_dir(prog: &str, facts_dir: &Path) {
    let_cxx_string!(prog = prog);
    let_cxx_string!(facts = facts_dir.to_string_lossy().as_bytes());
    let_cxx_string!(empty = "");

    let mut prog = ffi::ProgramFactory_newInstance(&prog);
    let mut prog = prog.as_mut().expect("Wrong program name");
    ffi::load_all(prog.as_mut(), &facts);
    prog.as_mut().run();
    ffi::print_all(prog.as_mut(), &empty);
}

pub fn run_from_facts<T>(prog: &str, facts: &AllFacts<T>) -> HashMap<String, DynTuples>
where
    T: FactTypes,
    T::Origin: Into<u32>,
    T::Loan: Into<u32>,
    T::Point: Into<u32>,
    T::Variable: Into<u32>,
    T::Path: Into<u32>,
{
    let_cxx_string!(prog = prog);

    let mut prog = ffi::ProgramFactory_newInstance(&prog);
    let mut prog = prog.as_mut().expect("Wrong program name");
    facts::insert_all_facts(prog.as_mut(), facts);
    prog.as_mut().run();
    facts::extract_output_facts(prog.as_mut())
}

