use std::pin::Pin;

pub use self::ffi::*;

#[cxx::bridge(namespace = "souffle")]
mod ffi {
    struct Tuple1 {
        a: u32,
    }

    struct Tuple2 {
        a: u32,
        b: u32,
    }

    struct Tuple3 {
        a: u32,
        b: u32,
        c: u32,
    }

    struct Tuple4 {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }

    /// A list of tuples whose size is only known at runtime.
    #[derive(Default)]
    struct DynTuples {
        data: Vec<u32>,
        arity: usize,
    }

    /// A pointer to a relation. Needed to work around a shortcoming in cxx.
    struct RelationPtr {
        ptr: *const Relation,
    }

    unsafe extern "C++" {
        include!("souffle/SouffleInterface.h");
        include!("polonius-souffle/shims/shims.hpp");

        #[cxx_name = "SouffleProgram"]
        type Program;

        fn ProgramFactory_newInstance(s: &CxxString) -> UniquePtr<Program>;

        fn load_all(prog: Pin<&mut Program>, dir: &CxxString);
        fn print_all(prog: Pin<&mut Program>, dir: &CxxString);
        fn getRelation(self: &Program, relation: &CxxString) -> *mut Relation; // FIXME: Should be `Option<&mut Relation>`
        fn run(self: Pin<&mut Program>);
        fn get_output_relations_raw(prog: &Program, relations: &mut Vec<RelationPtr>);

        type Relation;

        fn size(self: &Relation) -> usize;
        // fn getSignature(self: &Relation) -> UniquePtr<CxxString>;
        fn getArity(self: &Relation) -> u32;
        fn get_name(rel: &Relation) -> UniquePtr<CxxString>;

        fn insert_tuple1(prog: Pin<&mut Program>, rel: Pin<&mut Relation>, t: Tuple1);
        fn insert_tuple2(prog: Pin<&mut Program>, rel: Pin<&mut Relation>, t: Tuple2);
        fn insert_tuple3(prog: Pin<&mut Program>, rel: Pin<&mut Relation>, t: Tuple3);
        fn insert_tuple4(prog: Pin<&mut Program>, rel: Pin<&mut Relation>, t: Tuple4);

        fn dump_tuples(rel: &Relation) -> DynTuples;
    }
}

// Rust wrappers

impl Program {
    pub fn get_output_relations(&self) -> impl Iterator<Item = &'_ Relation> {
        let mut relations = vec![];
        get_output_relations_raw(self, &mut relations);

        relations.into_iter().map(|ptr| unsafe { &*ptr.ptr })
    }
}

impl DynTuples {
    pub fn iter(&self) -> std::slice::ChunksExact<'_, u32> {
        self.data.chunks_exact(self.arity)
    }
}

// Tuples

impl Tuple1 {
    pub fn insert_into_relation(self, prog: Pin<&mut Program>, rel: Pin<&mut Relation>) {
        insert_tuple1(prog, rel, self)
    }
}

impl Tuple2 {
    pub fn insert_into_relation(self, prog: Pin<&mut Program>, rel: Pin<&mut Relation>) {
        insert_tuple2(prog, rel, self)
    }
}

impl Tuple3 {
    pub fn insert_into_relation(self, prog: Pin<&mut Program>, rel: Pin<&mut Relation>) {
        insert_tuple3(prog, rel, self)
    }
}

impl Tuple4 {
    pub fn insert_into_relation(self, prog: Pin<&mut Program>, rel: Pin<&mut Relation>) {
        insert_tuple4(prog, rel, self)
    }
}

// Conversion method into FFI tuples.
//
// `From` or `Into` would be better, but this helps type deduction inside the fact loading macro.
pub trait IntoTuple<T> {
    fn into_tuple(self) -> T;
}

impl<A: Into<u32>> IntoTuple<Tuple1> for (A,) {
    fn into_tuple(self) -> Tuple1 {
        Tuple1 { a: self.0.into() }
    }
}

impl<A: Into<u32>, B: Into<u32>> IntoTuple<Tuple2> for (A, B) {
    fn into_tuple(self) -> Tuple2 {
        Tuple2 {
            a: self.0.into(),
            b: self.1.into(),
        }
    }
}

impl<A: Into<u32>, B: Into<u32>, C: Into<u32>> IntoTuple<Tuple3> for (A, B, C) {
    fn into_tuple(self) -> Tuple3 {
        Tuple3 {
            a: self.0.into(),
            b: self.1.into(),
            c: self.2.into(),
        }
    }
}

impl<A: Into<u32>, B: Into<u32>, C: Into<u32>, D: Into<u32>> IntoTuple<Tuple4> for (A, B, C, D) {
    fn into_tuple(self) -> Tuple4 {
        Tuple4 {
            a: self.0.into(),
            b: self.1.into(),
            c: self.2.into(),
            d: self.3.into(),
        }
    }
}

impl<A: From<u32>> From<Tuple1> for (A,) {
    fn from(t: Tuple1) -> Self {
        (t.a.into(),)
    }
}

impl<A: From<u32>, B: From<u32>> From<Tuple2> for (A, B) {
    fn from(t: Tuple2) -> Self {
        (t.a.into(), t.b.into())
    }
}

impl<A: From<u32>, B: From<u32>, C: From<u32>> From<Tuple3> for (A, B, C) {
    fn from(t: Tuple3) -> Self {
        (t.a.into(), t.b.into(), t.c.into())
    }
}

impl<A: From<u32>, B: From<u32>, C: From<u32>, D: From<u32>> From<Tuple4> for (A, B, C, D) {
    fn from(t: Tuple4) -> Self {
        (t.a.into(), t.b.into(), t.c.into(), t.d.into())
    }
}
