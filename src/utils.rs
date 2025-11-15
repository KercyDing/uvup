use std::io::{self, Write};

pub(crate) fn print_success(message: &str) {
    let _ = writeln!(io::stdout(), "{message}");
}

pub(crate) fn print_info(message: &str) {
    let _ = writeln!(io::stdout(), "{message}");
}
