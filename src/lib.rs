#[macro_use]
extern crate gtmpl;
extern crate gtmpl_value;
#[macro_use]
extern crate lazy_static;
extern crate data_encoding;
extern crate itertools;

mod strings;


use gtmpl::Func;

use std::collections::HashMap;

lazy_static! {
    /// Map of all builtin function.
    pub static ref SPRIG: HashMap<String, Func> = {
        let mut m = HashMap::new();
        m.insert("split".to_owned(), strings::split as Func);
        m
    };
}
