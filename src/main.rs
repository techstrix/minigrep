use::std::env;
use std::process;
use std::error::Error;
use minigrep::Config;
fn main() {
    let args:Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}",{err});
        process::exit(1);
    });

    println!("Searching for {} in {}",config.query,config.file_path);
    if let Err(e) = minigrep::run(config){
        eprintln!("Application Error {}",e);
        process::exit(1);
    }
   

}

