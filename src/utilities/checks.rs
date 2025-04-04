use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Checks {}

impl Checks {
    pub fn _is_numeric(text: &String) -> bool {
        text.parse::<f64>().is_ok()
    }

    pub fn _index_of_item_in_container<F>(container: &String, item: &char, func: F) -> i32
    where
        F: Fn(&char, &char) -> bool,
    {
        let mut index = -1;

        for c in container.chars() {
            if func(&c, item) {
                index += 1;
                break;
            }

            index += 1;
        }

        index
    }
}

pub fn exit_program(code: i32) {
    std::process::exit(code);
}
