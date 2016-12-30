#![feature(plugin)]
#![plugin(jsx_to_simpledom)]

pub use self::simpledom::{el, render_to_string, Element};

mod simpledom;