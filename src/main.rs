fn main() {

    //Scalar Types:

    //- integer:
    //8, 16, 32, 64, 128 (length: bits) and isize or usize (length: arch) [default: 32]
    //unsigned (only positive)
    let num1: u8 = 1; //(0 - 255)
    //signed (could be negative or positive)
    let num2: i8 = -1; //(-128 - 127)
    println!("{}", num1);
    println!("{}", num2);

    //-floating-point:
    //f32 and f64 (bits) [default: 64]
    let fp1 = 2.0; // f64
    let fp2: f32 = 3.0; // f32
    println!("{}", fp1);
    println!("{}", fp2);

    //Numeric Operations:
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;
    
        // multiplication
        let product = 4 * 30;
    
        // division
        let quotient = 56.7 / 32.2;
        let floored = 2 / 3; // Results in 0
    
        // remainder
        let remainder = 43 % 5;
}