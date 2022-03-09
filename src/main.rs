fn main() {

    //ownership

    //Ways Variables and Data Interact: Move

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}

/*
To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid. 
Therefore, Rust doesn’t need to free anything when s1 goes out of scope.
You’ll get an error like this because Rust prevents you from using the invalidated reference

If you’ve heard the terms shallow copy and deep copy while working with other languages, 
the concept of copying the pointer, length, and capacity without copying the data probably 
sounds like making a shallow copy. But because Rust also invalidates the first variable, 
instead of calling it a shallow copy, it’s known as a move. In this example, we would 
say that s1 was moved into s2.

That solves our problem! With only s2 valid, when it goes out of scope, it alone will free the memory, 
and we’re done.

In addition, there’s a design choice that’s implied by this: Rust will never automatically 
create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive 
in terms of runtime performance.
*/