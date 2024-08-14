use std::fs;
use::std::error::Error;


pub struct Config {
    pub file_name: String,
    pub search_word: String,
}
impl Config{
    pub fn new(arg: &Vec<String>)->Result<Config, &str>{
        if arg.len() < 3 {
            return Err("Not enough arguments");
        }
        let file_name=  arg[1].clone();
        let search_word = arg[2].clone();
        Ok(Config{
            file_name,
            search_word
        })
    }
}


pub fn read_file(arg: Config)-> Result<(), Box<dyn Error>>{
    let result = fs::read_to_string(arg.file_name)?;
    for value in get_value_from_string(&result, &arg.search_word){
        println!("{}", value);
    }
    Ok(())
}


pub fn get_value_from_string<'a>(string_to_check: &'a str, compare: & 'a str)-> Vec<&'a str>{
    let mut result: Vec<&str> = Vec::new();
    for line in string_to_check.lines(){
       if line.contains(compare) {
        result.push(line)
       }
    }
    result
}


pub fn case_sensitvie_get_value_from_string<'a>(string_to_check: &'a str, compare: &str)-> Vec<&'a str>{
    let mut result:Vec<&str> = Vec::new();
    for line in string_to_check.lines(){
        if line.to_lowercase().contains(&compare.to_lowercase()) {
            result.push(line.trim())
        }
    }
    result
}