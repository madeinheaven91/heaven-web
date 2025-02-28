use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref LEXICON: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("db_error", "An error occured while performing database operation");
        m
    };

}
