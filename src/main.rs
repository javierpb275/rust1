fn main() {

    //COMPOUND TYPES

    //Tuple
    //Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;//destructuring
    println!("The value of y is: {}", y);
    //We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("fiver hundred: {}", five_hundred);
    println!("6 point four: {}", six_point_four);
    println!("one {}", one);

    //Array
    //Unlike a tuple, every element of an array must have the same type but they also have a fixed length
    let a = [1, 2, 3, 4, 5];
    //specify type:
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    //You can also initialize an array to contain the same value for each element by specifying the initial value, 
    //followed by a semicolon, and then the length of the array in square brackets
    let c = [3; 5]; //[3, 3, 3, 3, 3]

    //Accessing array elements:
    let d = [1, 2, 3, 4, 5];
    let first = d[0];
    let second = d[1];
}