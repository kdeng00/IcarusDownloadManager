use std::default::Default;


pub struct FileManager {
    pub filepath: String,
    pub filebuffer: String,
    pub file_read: bool,
    pub file_buffer_length: i32,
}

impl Default for FileManager {
    fn default() -> Self {
        FileManager {
            filepath: String::new(),
            filebuffer: String::new(),
            file_read: false,
            file_buffer_length: -1
        }
    }
}


impl FileManager {
// TODO: Implement
pub fn init(&mut self) {
    self.read_file()
}
    pub fn save_file(&mut self, filepath: &String) {
        if !self.file_read {
            self.read_file();
        }

            let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
        self.file_buffer_length = buffer.len();
    self.filebuffer = String::from_utf8(buffer); // Assuming UTF-8 encoding
    }
    pub fn modify_file_path(&mut self, file: &String) {
        self.filepath = file;
    }

    pub fn retrieve_file_buffer(&self) -> String {
        self.filebuffer;
    }

    pub fn retrieve_file_length(&self) -> i32 {
        self.file_buffer_length;
    }

    fn read_file(&mut self) {
        let mut file = File::open(self.filepath)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        self.file_buffer_length = buffer.len();
        self.filebuffer = String::from_utf8(buffer); // Assuming UTF-8 encoding
        self.file_read = true;
    }
}