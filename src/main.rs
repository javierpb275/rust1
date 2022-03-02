fn main() {

    //control flow

    //loops with while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("--------------------------");

    //loops with for
    let b = [10, 20, 30, 40, 50];

    for element in b {
        println!("the value is: {}", element);
    }
}
