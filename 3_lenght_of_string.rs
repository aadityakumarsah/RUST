// Write a function getStringLength() that takes a string as an input and returns its length.

fn main(){
    let s: String = String::from("aaditya");
    println!("the lenght of the given string is {}", get_string_lenght(s));
}

fn get_string_lenght(s : String) -> usize{
    s.chars().count()
}