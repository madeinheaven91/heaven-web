use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref LEXICON: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("db_error", "An error occured while performing database operation");
        m.insert("db_pool_error", "Couldn't get database connection from pool");
        m
    };

}

pub fn slugify(original: &str) -> String {
    original
        .to_lowercase()
        .chars()
        .filter_map(|c|
                match c {
                    ' ' => Some("-".to_string()),
                    'а' => Some("a".to_string()),
                    'б' => Some("b".to_string()),
                    'в' => Some("v".to_string()),
                    'г' => Some("g".to_string()),
                    'д' => Some("d".to_string()),
                    'е' => Some("e".to_string()),
                    'ё' => Some("jo".to_string()),
                    'ж' => Some("zh".to_string()),
                    'з' => Some("z".to_string()),
                    'и' => Some("i".to_string()),
                    'й' => Some("j".to_string()),
                    'к' => Some("k".to_string()),
                    'л' => Some("l".to_string()),
                    'м' => Some("m".to_string()),
                    'н' => Some("n".to_string()),
                    'о' => Some("o".to_string()),
                    'п' => Some("p".to_string()),
                    'р' => Some("r".to_string()),
                    'с' => Some("s".to_string()),
                    'т' => Some("t".to_string()),
                    'у' => Some("u".to_string()),
                    'ф' => Some("f".to_string()),
                    'х' => Some("h".to_string()),
                    'ц' => Some("ts".to_string()),
                    'ч' => Some("ch".to_string()),
                    'ш' => Some("sh".to_string()),
                    'щ' => Some("sh".to_string()),
                    'ъ' => Some("j".to_string()),
                    'ы' => Some("i ".to_string()),
                    'ь' => Some("j".to_string()),
                    'э' => Some("e".to_string()),
                    'ю' => Some("ju".to_string()),
                    'я' => Some("ja".to_string()),
                    _ if c.is_alphanumeric() => Some(c.to_string()),
                    _ if c.is_control() || c.is_ascii_punctuation() => None,
                    _ => None
                }
        )
        .collect()
}
