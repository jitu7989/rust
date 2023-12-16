
use std::error::Error;
use std::{fs, thread,time::Duration};


pub struct Config{
    pub query: String,
    pub file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str> {

        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        let query = args[1].clone();
        let file = args[2].clone();
        Ok(Config{query,file})
    }
}
pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents  = fs::read_to_string(config.file).expect("Something went wrong reading the file");
    println!("With Test:\n {}",contents);


    Ok(())
}
fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {

    let mut result = vec![];
    for line in  content.lines() {
        if line.contains(query){
            result.push(line);
        }
    }
    result
}



#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn one_result(){

        let query = "duct";

        let contains = "\
Rust
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contains));


    }


}





