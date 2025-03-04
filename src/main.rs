use std::env;
use std::process;
use minigrep::run;
use minigrep::Config;
// use std::String;

fn main(){
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    
}
