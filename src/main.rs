fn main() {

    //ownership

    //memory and allocation

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
}

/*
Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
There is a natural point at which we can return the memory our String needs to the allocator: 
when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. 
This function is called drop, and it’s where the author of String can put the code to return the memory. 
Rust calls drop automatically at the closing curly bracket.
*/