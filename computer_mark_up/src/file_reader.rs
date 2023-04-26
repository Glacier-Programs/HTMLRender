/* 
 * This reads a file and then 
 * constructs a bunch of components
 * based off of it.
 * ezpz lemon squeezy
*/

use std::fs;

pub fn load_file(file_location: &str) -> String{
    fs::read_to_string(file_location)
        .expect("Should have been able to read the file")
}