pub mod encrypt_key {
    pub use crate::util::general;
    pub use crate::log::log_file;
    pub use crate::encryption::encryption;
    use std::io::Write;
    use std::fs;
    use chrono::Local;

    static ENCRYPT_MAINKEY_FILE_PATH: &str = "./files/mainkey.txt";
    static ENCRYPT_SUBKEY_FILE_PATH: &str = "./files/subkey.txt";
    static STRICT_FILE_STATE: bool = true;
    static RANDOM_FILE_STATE: bool = true;

    pub fn init_encrypt_key(){
        init_check_file();
        generate_mainkey();
        generate_subkey();
    }

    fn init_check_file(){
        let (exist,_filename) = general::check_file_create(ENCRYPT_MAINKEY_FILE_PATH,STRICT_FILE_STATE,RANDOM_FILE_STATE);
        if !exist {
            log_file::print_log("Created key file","info");
        }
    }

    pub fn generate_subkey(){
        // Used to create root subkey
        // 1. Create a root subkey from encrypting
        // 2. 

        let mut file = fs::OpenOptions::new().append(true).open(ENCRYPT_SUBKEY_FILE_PATH).unwrap();
        let num_of_lines = general::count_line_file(ENCRYPT_SUBKEY_FILE_PATH);
        let current_id = num_of_lines + 1;

        let salt = "";
        let encrypted_subkey = "";

        write!(&mut file,"{} {} {}", current_id, &encrypted_subkey, &salt).unwrap();
    }

    fn generate_mainkey(){
        let now = format!("{}",Local::now(),);
        let hash_key = general::hash_string(&now);
        let mut file = fs::OpenOptions::new().append(true).open(ENCRYPT_MAINKEY_FILE_PATH).unwrap();
        write!(&mut file,"{}",&hash_key).unwrap();
        log_file::print_log("Created main key", "info");
    }
    
    pub fn return_sub_key(token: &str) -> &str {
        let mut subkey = "debug";
        return subkey;
    }
    pub fn round_sub_key(){
        // 1. Create next sub key and print on file
        // 2. Decrypt main key
        // 3. Encrypt main key
        // 4. Put key on file, and encrypt the file with main key
        // 5. Print to Log file that the key is rounded
    }
    pub fn return_main_key(subkey: &str) -> &str {
        // 1. Verify token, and bring decrypt mainkey file with subkey
        // 2. Read the mainkey, then return it

        let mut mainkey = "debug";
        return mainkey;
    }
    pub fn round_main_key(){
        // 1. Create next main key and print on file
        // 2. Get subkey and decrypt the main key
        // 3. Decrypt all files 
        // 4. Encrypt all files with the new key
        // 5. Put key on file, and encrypt the file with subkey
    }
}