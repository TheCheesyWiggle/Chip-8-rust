#[derive(Debug)]
//Processor structure
struct Processor{
    //General registers
    general_registers : [u8;16],
    //Address register
    address_register : u16,
    //Stack
    stack : Vec<u16>,
}
//Register 15 or F
const VF:usize =15;



fn main(){
    println!("Hello, world!");
}

//runs a tests
#[cfg(test)]
mod test_super {
    use super::*;

    //runs a test
    #[test]
    fn test_registers_to_zero() {
        //sets registers to 0
        let registers = GeneralRegisters::default();
        //iterates through registers and throws error if they arent default
        registers.iter().for_each(|&r| assert_eq!(r, 0));
    }

    //runs a test
    #[test]
    fn test_vf_is_addressable() {
        //sets registers to 0
        let registers = GeneralRegisters::default();
        assert_eq!(registers[VF], 0);
    }
}