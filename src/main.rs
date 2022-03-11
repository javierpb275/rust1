fn main() {

    //References

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

/*
A reference is like a pointer in that it’s an address we can follow to access data 
stored at that address that is owned by some other variable. Unlike a pointer, 
a reference is guaranteed to point to a valid value of a particular type. 

These ampersands represent references, and they allow you to refer to 
some value without taking ownership of it.
*/