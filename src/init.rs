pub mod initial {
    pub fn init(){
        if(check_log()){
            
        }
    }

    fn check_log(){
        let file_path = Path::new("");
        if !(file_path.exists() || file_path.is_file()){
            return false;
        }
        return true;
    }
}