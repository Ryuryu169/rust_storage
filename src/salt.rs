pub mod salt {
    use std::fs;
    use std::io::{BufRead, BufReader,Write};
    use rand::{rngs::OsRng, RngCore};
    pub use crate::util::general;

    static SALT_FILE_PATH: &str = "./files/filename_salt.txt";
    static STRICT_FILE_STATE: bool = true;
    static RANDOM_FILE_STATE: bool = false;

    pub fn init_filename_salt(){
        // Generate file which contains filenames, restrict unwanted access
        // 
        general::check_file_create(SALT_FILE_PATH, STRICT_FILE_STATE, RANDOM_FILE_STATE);
    }
    pub fn add_salt(token: &str){
        let salt = return_salt(token);
        if salt == "" {
            let mut file = fs::OpenOptions::new().append(true).open(SALT_FILE_PATH).unwrap();
            let num_of_lines = general::count_line_file(SALT_FILE_PATH);
            let salt = generate_salt();
            write!(file,"{} {} {:?}",num_of_lines,token,salt).unwrap();
        }
        println!("Error: Salt already exists");
    }
    pub fn return_salt(token: &str) -> String {
        let file = fs::File::open(SALT_FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines(){
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            if token.eq(parts[1]){
                return format!("{}",parts[2]);
            }
        }
        return format!("");
    }
    pub fn generate_salt() -> [u8; 16] {
        let mut salt = [0u8; 16];
        let mut os_rng = OsRng;
        os_rng.fill_bytes(&mut salt);
        salt
    }
}