mod log;
mod init;
mod account_general;
mod key_master;
mod util;
mod encryption;
mod salt;
//pub use crate::log::log_file;
pub use crate::init::initial;

// Debug inports
//use std::fs;

pub fn main() {
    initial::init();
}