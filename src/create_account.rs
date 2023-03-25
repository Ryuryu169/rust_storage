pub mod account{
    pub use crate::util::general;
    use chrono::prelude::*;
    use std::io::{self, BufRead, BufReader};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::fs;

    pub fn create_new_account(id: &str, pass: &str){
        let path = "account.txt";
        let file_exist = general::check_file(path);

        let now: DateTime<Local> = Local::now();
        let msg = format!("{}&{}&{}",id,pass,now.format("%H:%M:%S"));
        let mut hasher = DefaultHasher::new();
        msg.hash(&mut hasher);
        let hash_value = hasher.finish();
        let hashmsg = hash_value.to_string();

        let exist = check_account_exist(&hashmsg);
        if exist {
            println!("Account already exists");
        }
        let mut file = fs::OpenOptions::new().append(true).open(path).unwrap();
        
    }
    fn check_account_exist(hash_value: &str) -> bool {
        return false;
    }
}