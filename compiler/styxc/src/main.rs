extern crate clap;
extern crate log;

use clap::{AppSettings, Clap};
use log::{debug, error, LevelFilter};
use std::fs;
use std::path::Path;
use std::process::exit;
use styxc::ast::{build_ast, validate_ast, Scope};

/// Execute styx files using the Styx JIT compiler.
#[derive(Clap)]
#[clap(version = "1.0", author = "Skye Elliot <actuallyori@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// The target file to execute.
    input: String,
    #[clap(short, long)]
    /// Enable verbose logging output.
    verbose: bool,
}

fn main() {
    // initialize environment logger
    let opts = Opts::parse();
    env_logger::builder()
        // set filter level depending on verbosity
        .filter_level(match opts.verbose {
            true => LevelFilter::Debug,
            false => LevelFilter::Info,
        })
        .init();

    let filepath = Path::new(&opts.input);
    // check if the target file exists.
    debug!("Checking if specified file exists...");
    if !filepath.exists() {
        error!("File '{}' does not exist!", opts.input);
        exit(1);
    }
    // print file name
    debug!("Target file is '{}'", opts.input);
    // read target file into memory
    let file = match fs::read_to_string(filepath) {
        Ok(f) => f,
        Err(e) => {
            error!("Error while reading file '{}'", opts.input);
            error!("{}", e);
            exit(1);
        }
    };
    // print code for debugging purposes
    debug!("Building the AST...");
    debug!("Code to compile:");
    if opts.verbose {
        println!("[PEG_INPUT_START]");
        println!("{}", file);
        println!("[PEG_TRACE_START]");
    }

    // build the AST
    let ast = match build_ast(file) {
        Ok(ast) => ast,
        Err(e) => {
            error!("Exception encountered while building the AST!");
            error!("{}", e);
            exit(1);
        }
    };
    // check the ast
    match validate_ast(&mut Scope::default(), ast) {
        Ok(_) => debug!("ast successfuly validated"),
        Err(e) => {
            error!("Exception encountered while validating the AST!");
            error!("{}", e);
            exit(1);
        }
    }
}
