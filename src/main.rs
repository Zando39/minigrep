pub use std::env;
pub use std::process;
pub use std::iter;

pub use minigrep::Config;

fn main() {
    let args = env::args();
    let config = Config::build(args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for \"{}\" in file {}\n", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
    }
}
