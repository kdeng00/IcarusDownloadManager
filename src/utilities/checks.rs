use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Checks {
}

impl Checks {
    pub fn is_number(val: &String) -> bool {
        return val.is_numeric();
    }

    // TODO: Implement
    pub fn item_in_container(container: &Vec<String>, item: &String,
        func: fn(a: &String, b: &String) -> String) -> String {
        return String::from("");
    }

    // TODO: Implement
    pub fn item_iter_in_container(container: &Vec<String>, item: &String,
        func: fn(a: &String, b: &String) -> String) -> String {
        return String::from("");
    }
}
