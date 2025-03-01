use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::config::Config;

lazy_static!{
    pub static ref LEXICON: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("db_error", "An error occured while performing database operation");
        m.insert("db_pool_error", "Couldn't get database connection from pool");
        m.insert("startup", 
r#"
 _                                                   _     
| |                                                 | |    
| |__   ___  __ ___   _____ _ __ ________      _____| |__  
| '_ \ / _ \/ _` \ \ / / _ \ '_ \______\ \ /\ / / _ \ '_ \ 
| | | |  __/ (_| |\ V /  __/ | | |      \ V  V /  __/ |_) |
|_| |_|\___|\__,_| \_/ \___|_| |_|       \_/\_/ \___|_.__/ "#
        );
        m
    };

    pub static ref CONFIG: Config = Config::init();
}
