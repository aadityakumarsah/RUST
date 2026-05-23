// Write a function is_even that takes a number as an input and returns true if it is even.

fn main(){
    is_even(6);
}

fn is_even(num : i32){
    if num % 2 == 0{
        println!("the number is even");
    }
    else {
        println!("the number is odd");
    }
}

