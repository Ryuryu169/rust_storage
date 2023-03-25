pub mod general{
    use std::path::Path;
    use std::fs::File;
    pub use crate::log::log_file;
    pub fn check_file(path: &str) -> bool {
        let file_path = Path::new(path);
        if !(file_path.exists() || file_path.is_file()){
            let f = File::create(file_path);
            match f {
                Ok(_) => println!("Created file"),
                Err(_) => println!("Unknown error"),
            }
            return false;
        }
        return true;
    }
}