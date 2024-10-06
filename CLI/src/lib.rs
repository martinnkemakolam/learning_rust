use std::fs;
use::std::error::Error;
use::std::env;

pub struct Config {
    pub file_name: String,
    pub search_word: String,
    pub case_sensistive: bool
}
impl Config{
    pub fn new(mut arg: env::Args)->Result<Config, &'static str>{
        // if arg.len() < 3 {
        //     return Err("Not enough arguments");
        // }
        arg.next();
        let file_name=  match arg.next(){
            Some(arg)=> arg,
            None => return Err("Didn't find file")
        };
        let search_word = match arg.next() {
            Some(arg)=>arg,
            None => return Err("Didn't find Search word") 
        };
        let case_sensistive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config{
            file_name,
            search_word,
            case_sensistive
        })
    }
}


pub fn read_file(arg: Config)-> Result<(), Box<dyn Error>>{
    let result = fs::read_to_string(arg.file_name)?;
    let result = if arg.case_sensistive {
        for value in get_value_from_string(&result, &arg.search_word){
            println!("{}", value);
        }
    } else {
        for value in case_sensitvie_get_value_from_string(&result, &arg.search_word){
            println!("{}", value);
        }
    };
    Ok(())
}


pub fn get_value_from_string<'a>(string_to_check: &'a str, compare: & 'a str)-> Vec<&'a str>{
    // let mut result: Vec<&str> = Vec::new();
    // for line in string_to_check.lines(){
    //    if line.contains(compare) {
    //     result.push(line)
    //    }
    // }
    string_to_check.lines().filter(|line| line.contains(compare)).collect()
}


pub fn case_sensitvie_get_value_from_string<'a>(string_to_check: &'a str, compare: &str)-> Vec<&'a str>{
    // let mut result:Vec<&str> = Vec::new();
    // for line in string_to_check.lines(){
    //     if line.to_lowercase().contains(&compare.to_lowercase()) {
    //         result.push(line.trim())
    //     }
    // }
    // result
    string_to_check.lines().filter(|line| line.to_lowercase().contains(&compare.to_lowercase())).collect()
}