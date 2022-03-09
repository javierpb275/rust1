fn main() {

    //ownership

    //string type

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    /*
    The double colon :: operator allows us to namespace this particular from function 
    under the String type rather than using some sort of name like string_from. 
    when we call String::from, its implementation requests the memory it needs. 
    This is pretty much universal in programming languages.
    */
}
