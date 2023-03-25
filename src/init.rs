pub mod initial {
    pub use crate::create_account::account;
    pub use crate::util::general;
    use std::fs;
    use std::io::prelude::*;
    use std::path::Path;
    use chrono::prelude::*;

    pub fn init(){
        let first = general::check_file("acount.txt");
        if !first {
            // Functions for initialize

        }
        account::create_new_account("kaname", "reweti");
        println!("Welcome to kaname Database!!!\n");
    }
}