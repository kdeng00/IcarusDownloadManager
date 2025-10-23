pub fn o_to_string(val: &std::ffi::OsString) -> Result<std::string::String, std::io::Error> {
    match val.clone().into_string() {
        Ok(value) => Ok(value),
        Err(err) => match err.into_string() {
            Ok(res) => {
                Err(std::io::Error::other(res.to_string()))
            }
            Err(err) => {
                match err.to_str() {
                    Some(err) => {
                        Err(std::io::Error::other(err))
                    }
                    None => {
                        Err(std::io::Error::other("Undefined error"))
                    }
                }
            }
        }
    }
}
