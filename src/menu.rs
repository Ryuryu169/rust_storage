pub mod showmenu{
    use std::io::{self,Write};

    pub fn menu(){
        let mut exit = false;
        println!("Hello!");
        while !exit {
            let mut user_input: String = String::new();
            print!("KanaDB > ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut user_input).unwrap();
            user_input = user_input.trim().to_string();
            check_input(user_input);
            exit = user_input == "exit";
        }
    }
    fn check_input(user_input:String){
        let usr_txt: &str = &user_input;
        match usr_txt {
            "choose" => 
            "view" => println!("Hello World!"),
        }
    }
}