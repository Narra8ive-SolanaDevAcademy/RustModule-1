// Fix all errors without adding newline
fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str("world");

    s += "!";
    println!("{:?}", s)
}

// push is useed to add single character in a string
// push_str() is used to add slice of string in the end of the string

/*
 Hint:  read the difference between push and push_str
*/
