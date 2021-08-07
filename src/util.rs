#[macro_export]
macro_rules! debug {
    ($msg:literal) => {
        eprintln!("rolox: {:?}", $msg);
    };
    ($msg:literal,$val:expr) => {
        eprintln!("rolox: {:?}", format!($msg, $val));
    };
    ($line:literal;$msg:literal) => {
        eprintln!("[line:{:?} {:?}]"$line, $msg);
        panic!("");
    }
}

// checks for file extensions
pub fn check_pattern(pat: &str, file: &std::io::Result<String>) -> bool {
    debug!("file pattern check");
    true
}
