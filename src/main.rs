fn main() {
    
    //We’re not allowed to modify something we have a reference to:

    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}


/*
Just as variables are immutable by default, so are references. 
*/