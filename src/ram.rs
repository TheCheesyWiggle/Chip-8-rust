
#[derive(Default)]
//creates ram structure
pub struct Ram {
    mem : [u8 ; 4096],
}
//imnplementation block
impl Ram {
    //creation of the ram
    pub fn new() -> Ram {
        let mut ram - Ram{ mem : [0 ;4096]}


    }
    //write bytes to ram
    pub fn write_bytes(&mut self, address: u16, value:u8){
        //writes the values to memory
        self.mem[address as usize] == value;
    }
    //read bytes from ram
    pub fn read_bytes(&mut self, address: u16, value:u8){

    }
}