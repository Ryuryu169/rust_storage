/// Usage: use crate::log::log_file;
/// print_log(path,message,id);

pub mod log_file{
    pub use crate::util::general;
    use std::fs;
    use std::io::prelude::*;
    use chrono::prelude::*;

    static LOG_FILE_PATH: &str = "./files/log.txt";
    static STRICT_FILE_STATE: bool = false;
    static RANDOM_FILE_STATE: bool = false;

    pub fn print_log(message: &str, id: &str){
        general::check_file_create(LOG_FILE_PATH, STRICT_FILE_STATE, RANDOM_FILE_STATE);
        let mut file = fs::OpenOptions::new().append(true).open(LOG_FILE_PATH).unwrap();
        let now: DateTime<Local> = Local::now();
        write!(&mut file, "{} | {id: <24}  | {}\n", now, message).unwrap();
        // Formatted file to print: (Current time) | (Formatted id) | (message)
    }
}