//Crates Imported
use std::fs::File;
use std::io::{read, Read};
use chip8::Chip8;
//Modules used
mod ram;
mod chip8;

fn main(){
    //opens game file
    let mut file = File::open("games/INVADERS").unwrap();
    //creates new vector for file data
    let mut data = Vec::<u8>::new();
    //read the file to the data vector
    file.read_to_end(&mut data);
    //creates a chip8 object
    let mut chip8 = Chip8::new();
    //loads game on the chip8 virtual machine
    chip8.load_rom(&data);


}