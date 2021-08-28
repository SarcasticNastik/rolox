mod lexer;
mod parser;
mod util;

use crate::lexer::Lexer;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::error::Error;
use structopt::StructOpt;
use util::check_pattern;

// Returns tokens as the following type
pub type Result<T> = std::result::Result<T, String>;

// storing repl history
const HISTORY_FILE_NAME: &'static str = "history.txt";
const PATTERN: &'static str = "*.rl";
static mut HAS_ERROR: bool = false;

// Input structure for our CLI tool
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str), default_value = "")]
    path: std::path::PathBuf,
}

// `THE COMPILER`
fn compile(lines: &String) {
    println!("{}", lines);
    // Lexer
    let lexer = Lexer::new(lines);

    for token in lexer.into_iter() {
        match token {
            Ok(t) => println!("{:?}", t),
            Err(e) => {
                unsafe {
                    HAS_ERROR = true;
                }
                debug!("lexer: {}", e);
            }
        }
    }

    //  Lexer -> Parser -> AST (would have to store some sort of global state)
}

// Read-Eval-Print-Loop
fn repl() {
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
            }
            Err(ReadlineError::Interrupted) => {
                debug!("Ctrl-C interrupt");
            }
            Err(ReadlineError::Eof) => {
                debug!("Ctrl-D interrupt");
                break;
            }
            Err(_) => {
                debug!("Error occurred");
                break;
            }
        }
        rl.save_history(HISTORY_FILE_NAME).unwrap();
    }
}

// Compiler case for
fn run(name: &str, content: &String) {
    debug!("Compiling file {}", name);
    compile(content);
    unsafe {
        if HAS_ERROR {
            debug!("Error while parsing/ compiling file");
            std::process::exit(65);
        }
    }
}

fn main() {
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
    let mat = check_pattern(&PATTERN, &file);
    if !mat {
        debug!("Filetype mismatch");
        // find some idiomatic way of refactoring this
        std::process::exit(1);
    }

    let file_name = ""; // TODO: Add function for filename
    run(file_name, &contents); // Compile the given program
}
