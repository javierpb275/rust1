fn main() {

    //Functions

    println!("hello world!");

    another_function(3);

    print_labeled_measurement(5, 'h');

}

fn another_function(x: i32) {
    println!("value of x is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}