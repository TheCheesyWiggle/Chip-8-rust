//Crates Imported
use std::fs::File;
use std::io::{read, Read};

fn main(){
    //opens game file
    let mut file = File::open("games/INVADERS").unwrap();
    //creates new vector for file data
    let mut data = Vec::<u8>::new();
    //read the file to the data vector
    file.read_to_end(&mut data);


}