fn main() {

    //Functions

    println!("hello world!");

    //parameter

    another_function(3);

    print_labeled_measurement(5, 'h');

    //return

    let x = five();

    println!("The value of x is: {}", x);

    let y = plus_one(5);

    println!("The value of y is: {}", y);

}

//functions with parameters

fn another_function(x: i32) {
    println!("af: the value is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

//functions with return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

//This function would give an error:
fn plus_one_error(x: i32) -> i32 {
    x + 1;
}

//if we add the ";" at the end we say that it is a statement and therefore we do NOT return the value.
//if we do NOT add the ";" it is an expression and we RETURN the value of that expression.
//you could use the keyword "return".