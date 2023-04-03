pub mod initial {
    pub use crate::account_general::create_account;
    pub use crate::key_master::encrypt_key;
    pub use crate::salt::salt;
    pub use crate::util::general;
    pub use crate::log::log_file;
    use std::process::Command;
    /*use std::fs;
    use std::io::prelude::*;
    use std::path::Path;
    use chrono::prelude::*;*/

    static DEFAULT_ID: &str = "info";
    static LOG_PATH: &str = "./files/log.txt";
    static STRICT_FILE_STATE: bool = true;
    static RANDOM_FILE_STATE: bool = true;

    pub fn init(){
        let output = Command::new("rm -r log.txt account.txt account_key.txt")
                    .output()
                    .expect("Failed to execute command");

        println!("{:#?}", &output);
        let (first,_filename) = general::check_file_create(LOG_PATH,STRICT_FILE_STATE,RANDOM_FILE_STATE);
        if !first {
            // Functions for initialize
            log_file::print_log("Created log file", DEFAULT_ID);
            salt::init_filename_salt();
            create_account::init_account();
            encrypt_key::init_encrypt_key();
        }
        println!("Welcome to kaname Database!!!\n");
    }
}