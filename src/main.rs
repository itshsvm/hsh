pub mod hsc;
pub mod variables;
pub mod shell;
pub mod errors;
use crate::shell::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("💥 Error: {}", e);
    }
}
