pub mod create_account{
    pub use crate::util::general;
    pub use crate::log::log_file;
    use std::io::{BufRead, BufReader,Write};
    use std::fs;

    static SALT_FILE_PATH: &str = "./files/salt.txt";
    static ENCRYPT_MAINKEY_FILE_PATH: &str = "./files/mainkey.txt";
    static ENCRYPT_SUBKEY_FILE_PATH: &str = "./files/subkey.txt";
    static STRICT_FILE_STATE: bool = true;
    static RANDOM_FILE_STATE: bool = true;

    pub fn create_new_account(id: &str, pass: &str){
        let hash_id = general::hash_string(id);
        let exist = check_account_exist(&hash_id);

        if exist {
            println!("Account already exists\n");
            return;
        }
        //let now: DateTime<Local> = Local::now();
        //now.format("%H:%M:%S")
        let msg = format!("{}&{}",id,pass);
        let hash_msg = general::hash_string(&msg);

        let mut file = fs::OpenOptions::new().append(true).open(SALT_FILE_PATH).unwrap();
        let num_of_lines = general::count_line_file(SALT_FILE_PATH);
        let current_id = num_of_lines + 1;

        write!(&mut file, "{} {} {}\n", current_id, hash_id, hash_msg).unwrap();

        let log_message = format!("Created account : {}", id);
        log_file::print_log(&log_message, id);
    }

    // Check whether the account exists
    // 1. 
    fn check_account_exist(hash_id: &str) -> bool {
        let file = fs::File::open(SALT_FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines(){
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            if hash_id.eq(parts[1]){
                return true;
            }
        }
        return false;
    }

    // Function to initialize and add root account
    pub fn init_account(){
        let root_id = "root";
        let root_pass = "root";
        let (tmp, filename) = general::check_file_create(SALT_FILE_PATH,STRICT_FILE_STATE,RANDOM_FILE_STATE);
        create_new_account(root_id, root_pass);
    }
}