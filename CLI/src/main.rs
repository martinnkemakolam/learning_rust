use std::{env, vec};
use::std::process;
use ::CLI::*;

// #[derive(Debug)]


fn main() {
    let environment_variables: Vec<String> = env::args().collect();
    let value = Config::new(&environment_variables).unwrap_or_else(
        |err|{
            println!("Program ended with error: {}", err);
            process::exit(1);
    });
    // if let Err(e) = read_file(&value.file_name) {
    //     println!("Application ended with error: {}", e);
    // };
    read_file(value).unwrap_or_else(
        |err|{
            println!("Application ended with error: {}", err);
            process::exit(1)
        }
    );
    // println!("This is value {:#?}",value);
}




#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn should_get_value_from_string() {
        let compare = vec!["Value 1"];
        let value_to_check = "\
        Value 1
        value";
        assert_eq!( compare, get_value_from_string(value_to_check, "Value"));
    }

    #[test]
    fn should_get_valuse_from_string_case_sensitive() {
        let compare = vec!["Value 1", "value"];
        let value_to_check = "\
        Value 1
        value";
        assert_eq!(compare, case_sensitvie_get_value_from_string(value_to_check, "value"))
    }
}
