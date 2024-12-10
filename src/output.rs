// output.rs
use crate::yd::TranslationError;

pub fn handle_error(result: Result<String, TranslationError>) -> i32 {
    match result {
        Ok(s) => {
            println!("{}", s);
            0
        }
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}
