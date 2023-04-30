/* 
 * This reads a file and then 
 * constructs a bunch of components
 * based off of it.
 * ezpz lemon squeezy
*/

use std::fs;

pub fn load_file(file_location: &str) -> String{
    match fs::read_to_string(file_location){
        Ok(string) => { string },
        Err(_) => { panic!("Something went oopsie loading a file!") }
    }
}

pub fn read_file(file_data: String){
    
}