extern crate clap;
extern crate log;

use std::env;
use std::path::Path;

use clap::{AppSettings, Clap};
use log::{debug, error, LevelFilter};

use fluxc_main::Mode;

/// Execute styx files using the Flux JIT compiler.
#[derive(Clap)]
#[clap(version = "1.0", author = "Skye Elliot <actuallyori@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// The target file to execute.
    input: String,
    #[clap(short, long)]
    /// Enable verbose logging output.
    verbose: bool,
    // Enable trace logging output.
    #[clap(short, long)]
    trace: bool,
    /// The output directory for the generated binary files. If this is set,
    /// the compiler is set to AOT mode. Defaults to the name of the target file
    /// without an extension.
    #[clap(short, long)]
    output: Option<String>,
    /// The compiler mode to use, defaults to JIT.
    #[clap(short, long, default_value = "jit")]
    mode: String,
}

fn main() {
    // initialize environment logger
    let opts = Opts::parse();
    // initialize logger
    let mut builder = env_logger::builder();
    if opts.verbose {
        builder.filter_level(LevelFilter::Debug);
    }
    if opts.trace {
        builder.filter_level(LevelFilter::Trace);
    }
    builder.init();
    // print splash
    debug!("styxc version {}", env!("CARGO_PKG_VERSION"));
    // lookup input path
    let input = Path::new(&opts.input);
    // check if input doesn't exist
    debug!("Cheking if input path exists...");
    if !input.exists() {
        error!("Input file does not exist: {:?}", input);
        return;
    }
    // fetch compiler mode
    match opts.mode.to_ascii_lowercase().as_str() {
        "jit" => {
            debug!("Compiling using JIT mode");
            fluxc_main::compile(input, Mode::JIT)
                .map_err(|e| error!("Error compiling: {:?}", e))
                .unwrap();
        }
        "aot" => {
            debug!("Compiling using AOT mode");
            let output_path = match opts.output {
                Some(path) => path,
                None => input.file_stem().unwrap().to_str().unwrap().into(),
            };
            let output = Path::new(&output_path);
            fluxc_main::compile(input, Mode::AOT(output)).unwrap();
        }
        _ => {
            error!("Unrecognized compiler mode '{}'", opts.mode);
        }
    };
}
