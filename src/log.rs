/// Usage: use crate::log::log_file;
/// print_log(path,message,id);

pub mod log_file{
    use std::fs;
    use std::io::prelude::*;
    use chrono::prelude::*;
    pub use crate::util::general;

    pub fn print_log(path: &str, message: &str, id: &str){
        general::check_file(path);
        let mut file = fs::OpenOptions::new().append(true).open(path).unwrap();
        let now: DateTime<Local> = Local::now();
        write!(&mut file, "{} {}    {}\n",now,id,message).unwrap();
    }
}