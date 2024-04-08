use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Conversions {
}

impl Conversions {
    pub fn to_lower_char(val: &mut char) {
        if val.is_alphabetic() {
            val = val.to_lowercase();
        }
    }

    pub fn initialize_values(&self) {
    }

    pub fn print_value<T>(&self, val: T) {
        println!("Going to print value");
        println!("{}", val);
    }
}