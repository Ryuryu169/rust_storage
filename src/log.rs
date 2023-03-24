/// Usage: use crate::log::log_file;
/// print_log(path,message,id);

pub mod log_file{
    use std::fs;
    use std::io::prelude::*;
    use std::path::Path;
    use chrono::prelude::*;

    pub fn print_log(path: &str, message: &str, id: &str){
        check_file(path,id);
        let mut file = fs::OpenOptions::new().append(true).open(path).unwrap();
        let now: DateTime<Local> = Local::now();
        write!(&mut file, "{} {}    {}\n",now,id,message).unwrap();
    }

    pub fn check_file(path: &str, id: &str){
        let file_path = Path::new(path);
        if !(file_path.exists() || file_path.is_file()){
            let f = fs::File::create(file_path);
            match f {
                Ok(_) => print_log(path,"Created log file",id),
                Err(_) => println!("Unknown error"),
            }
        }
    }
}