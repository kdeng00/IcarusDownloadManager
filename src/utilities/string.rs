
pub fn o_to_string(val: &std::ffi::OsString) -> Result<std::string::String, std::io::Error> {
    match val.clone().into_string() {
        Ok(value) => Ok(value),
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            String::from("Error"),
        )),
    }
}
