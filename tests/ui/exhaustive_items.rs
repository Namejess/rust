// run-rustfix

#![deny(clippy::exhaustive_enums, clippy::exhaustive_structs)]
#![allow(unused)]

fn main() {
    // nop
}

pub mod enums {
    pub enum Exhaustive {
        Foo,
        Bar,
        Baz,
        Quux(String),
    }

    // no warning, already non_exhaustive
    #[non_exhaustive]
    pub enum NonExhaustive {
        Foo,
        Bar,
        Baz,
        Quux(String),
    }

    // no warning, private
    enum ExhaustivePrivate {
        Foo,
        Bar,
        Baz,
        Quux(String),
    }

    // no warning, private
    #[non_exhaustive]
    enum NonExhaustivePrivate {
        Foo,
        Bar,
        Baz,
        Quux(String),
    }
}

pub mod structs {
    pub struct Exhaustive {
        foo: u8,
        bar: String,
    }

    // no warning, already non_exhaustive
    #[non_exhaustive]
    pub struct NonExhaustive {
        foo: u8,
        bar: String,
    }

    // no warning, private
    struct ExhaustivePrivate {
        foo: u8,
        bar: String,
    }

    // no warning, private
    #[non_exhaustive]
    struct NonExhaustivePrivate {
        foo: u8,
        bar: String,
    }
}
