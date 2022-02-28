fn main() {
    //variables:
    let y = 1;//inmutable variable
    let mut x = 5;//mutable variable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //constants:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //shadowing
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {}", z);
    }
    println!("The value of z is: {}", z);
}