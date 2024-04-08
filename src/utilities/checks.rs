use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Checks {
}

impl Checks {
    // TODO: Implement
    pub fn is_number(val: &String) -> bool {
        return false;
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
