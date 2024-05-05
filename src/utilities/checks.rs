use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Checks {}

impl Checks {
    pub fn is_numeric(text: &String) -> bool {
        text.parse::<f64>().is_ok()
    }

    // TODO: Implement
    pub fn item_in_container(
        container: &Vec<String>,
        item: &String,
        func: fn(a: &String, b: &String) -> String,
    ) -> String {
        return String::from("");
    }

    // TODO: Implement
    pub fn item_iter_in_container(
        container: &Vec<String>,
        item: &String,
        func: fn(a: &String, b: &String) -> String,
    ) -> String {
        return String::from("");
    }

    // TODO: Implement
    pub fn index_of_item_in_container<F>(
        container: &String,
        item: &char,
        func: F,
    ) -> i32 
    where F: Fn(&char, &char) -> bool,
    {
        let mut index = -1;

        for c in container.chars() {
            if func(&c, item) {
                break;
            }

            index += 1;
        }

        return index;
    }
}
