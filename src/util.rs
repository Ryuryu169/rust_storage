pub mod general{
    use std::hash;
    use std::path::{Path,PathBuf};
    use std::fs::{File,OpenOptions,remove_file};
    use std::io::{BufReader, BufRead,BufWriter,Write};
    use std::os::unix::fs::OpenOptionsExt;
    use rand::{Rng, thread_rng};
    use digest::Digest;
    use sha2::Sha256;
    use hex;
    pub use crate::secure::salt::salt;
    pub use crate::secure::encryption::encryption;
    pub use crate::log::log_file;

    // use self::encryption::encrypt_file;

    pub fn check_file_create(path: &str, strict_file: bool, random_file_name: bool) -> (bool, String) {
        let mut file_path = PathBuf::from(path);
        let mut filename = Path::new(path).file_name().unwrap().to_str().unwrap().to_owned();
        if !(file_path.exists() || file_path.is_file()){
            if random_file_name {
                let parent = file_path.parent().expect("Invalid file path");
                let salt = salt::generate_salt();
                let file_string = format!("{}_{:?}",filename,salt);
                filename = file_string;
                file_path = parent.join(&filename);
            }
            if strict_file {
                let _f = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .mode(0o600) 
                    .open(file_path)
                    .expect("Failed to open file");
                println!("Created file");

            } else {
                if let Ok(_f) = File::create(file_path) {
                    println!("Created file");
                } else {
                    println!("Unknown error");
                }
            }
            return (false,filename);
        }
        return (true,filename);
    }

    pub fn check_file_exist(path: &str) -> bool {
        let file_path = Path::new(path);
        if !(file_path.exists() || file_path.is_file()){
            return false;
        }
        return true;
    }

    pub fn count_line_file(path: &str) -> i32 {
        let file = BufReader::new(File::open(path).unwrap());
        let mut cnt = 0;
        for _ in file.lines(){
            cnt = cnt+1;
        }
        return cnt;
    }
    // function to generate hash_value, returns String
    pub fn hash_string(original_string: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(original_string.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
    pub fn secure_delete_file(file_path: &str) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .open(file_path)?;
        let file_size = file.metadata()?.len();
        let mut buffer = BufWriter::new(&file);
        let mut rng = thread_rng();
        const NUM_PASSES: i32 = 3;
        for i in 0..NUM_PASSES {
            println!("Pass {} of {}", i + 1, NUM_PASSES);
            for _ in 0..file_size {
                buffer.write(&[rng.gen::<u8>()])?;
            }
            buffer.flush()?;
        }
        remove_file(file_path)?;
        Ok(())
    }
    pub fn return_hashed_filename(original_filename: &str, salt: &str) -> String {
        // let salt = salt::return_salt(original_filename);
        let salted_filename = format!("{}_{}",original_filename,&salt);
        let hashed_filename = hash_string(&salted_filename);
        let path = format!("{}",hashed_filename);
        return path;
    }
}