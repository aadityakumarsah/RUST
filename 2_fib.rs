// Write a function fib that finds the Fibonacci of a number it takes as input.
//  0, 1, 2, 3,5, 8
// n = 1

fn main(){
    println!("{}", fib(5))
}


fn fib(n : i32) -> i32{
    if n == 0{
        return 0;
    }
    if n == 1{
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}