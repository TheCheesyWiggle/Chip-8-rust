//creates ram structure
struct Ram {
    mem : [u8 ; 4096],
}
//imnplementation block
impl Ram {
    //creation of the ram
    fn new() -> Ram {
        Ram{
            mem : [0 ,4096],
        }
    }
    //write bytes to ram
    fn write_bytes(&mut self, address: u16, value:u8){

    }
    //read bytes from ram
    fn read_bytes(&mut self, address: u16, value:u8){

    }
}