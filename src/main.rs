mod log;
mod init;
mod secure;
mod util;
pub use crate::log::log_file;
pub use crate::init::initial;
mod menu;
pub use crate::menu::showmenu;

// Debug inports
//use std::fs;

pub fn main() {
    //initial::init();
    showmenu::menu();
}