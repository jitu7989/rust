use std::env;
use std::process;

use grep::Config;


fn main() {
    let args: Vec<String>  = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem Parsing arguments: {}",err);
        process::exit(1);
    });

    if let Err(e) = grep::run(config){
        println!("Application Error: {}",e);
        process::exit(1);
    }

}

