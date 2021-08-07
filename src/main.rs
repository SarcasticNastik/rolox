use structopt::StructOpt;
use std::error::Error;
use rustyline::error::ReadlineError;
use rustyline::Editor;

// storing repl history
const HISTORY_FILE_NAME: &'static str = "history.txt";
const PATTERN: &'static str = "*.rl";

macro_rules! debug {
    ($msg:literal) => {
        eprintln!("rolox: {:?}", $msg);
    };
    ($msg:literal,$val:expr) => {
        eprintln!("rolox: {:?}", format!($msg, $val));
    };
    ($line:literal;$msg:literal) => {
        eprintln!("[line:{:?} {:?}]"$line, $msg);
    }
}

// Input structure for our CLI tool
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str), default_value="")]
    path: std::path::PathBuf,
}

// checks for file extensions
fn check_pattern(pat: &str, file: &std::io::Result<String>) -> bool
{
    debug!("file pattern check");
    true
}

// `THE COMPILER`
fn compile(lines: &String)
{
    println!("{}", lines);
}

// Read-Eval-Print-Loop
fn repl()
{
    debug!("Starting REPL for lox language");
    let mut rl = Editor::<()>::new();
    if rl.load_history(HISTORY_FILE_NAME).is_err() {
        println!("rolox: Failed to load history");
    }
    // repL
    loop {
        // Repl
        let readline = rl.readline("> ");
        match readline {
            Ok(ref line) => {
                rl.add_history_entry(line);
                // rEPl
                compile(line);
            },
            Err(ReadlineError::Interrupted) => {
                debug!("Ctrl-C interrupt");
                break;
            }
            Err(ReadlineError::Eof) =>
            {
                debug!("Ctrl-D interrupt");
                break;
            }
            Err(e) => {
                debug!("Error occurred");
                break;
            }
        }
        rl.save_history(HISTORY_FILE_NAME).unwrap();
    }
}

// Compiler case for
fn run(name: &str, content: &String)
{
    debug!("Compiling file {}", name);
    compile(content);
}

fn main() {
    let has_error = false;
    let args = Cli::from_args();
    // Interpreter
    if args.path == std::path::PathBuf::from("") {
        // Start the interactive read-eval-print loop
        repl();
        debug!("Ending the repl loop");
        std::process::exit(0);
    }

    // Compile the given `file`
    let file = std::fs::read_to_string(&args.path);
    let contents = match file {
        Ok(ref content) => content,
        Err(e) => {
            debug!("Error occurred while reading file contents: {}", e);
            std::process::exit(1);
        }
    };

    // otherwise compile the given file
    let mat = check_pattern(&pattern, &file);
    if !mat {
        debug!("Filetype mismatch");
        // find some idiomatic way of refactoring this
        std::process::exit(1);
    }

    let file_name = ""; // TODO: Add function for filename
    run(file_name, &contents); // Compile the given program
}
