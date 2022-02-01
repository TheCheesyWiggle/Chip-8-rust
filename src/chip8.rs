//imports ram
use ram::Ram;
//creates chip blue print
struct chip8{
    ram: Ram,
}
//implements chip
impl chip8 {
    //creates new chip
    pub fn new() -> chip8{
        chip8{
            ram: Ram::new(),
        }
    }
    //loads game data up
    pub fn load_rom(&mut self, data: &Vec<u8>) {
        //creates offest variable which offests the game data int the memory to leave room for the interpreter
        let offset = 0x200;
        //loops trhough data adding it to memory
        for i in 0..data.len(){
            //writes data to ram
            //data[i] == current byte index
            //adds 1 to the offset so it doesn't overwrite the ram which the interpreter is stored
            self.ram.write_byte((offset+1) as u16, data[i]);
        }
    }
}