mod log;
mod init;
mod create_account;
mod util;
pub use crate::log::log_file;
pub use crate::init::initial;

pub fn main() {
    // Initialize the database first
    initial::init();
}