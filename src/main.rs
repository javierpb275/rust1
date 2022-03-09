fn main() {

    //ownership

    //Stack-Only Data: Copy

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

/*
types such as integers that have a known size at compile time are stored entirely on the stack, 
so copies of the actual values are quick to make. That means there’s no reason we would want to 
prevent x from being valid after we create the variable y. In other words, there’s no difference 
between deep and shallow copying here, so calling clone wouldn’t do anything different from the 
usual shallow copying and we can leave it out.

Rust has a special annotation called the Copy trait that we can place on types that are stored 
on the stack like integers are. If a type implements the Copy trait, a variable is still valid 
after assignment to another variable.
*/