use std::io;
fn main() {
    let mut a = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    let a: i32 = a.trim().parse().expect("Please type a number!");
    let b = fibonacci(a);
    println!("{}",b);
}
fn fibonacci(n:i32)-> i32{
    let mut x =1;
    let mut y =1;
    let mut count =2;
    if n <= 0 {
        println!("please enter a positive integer!");
        return -1;
    }
    else if n<=2 {
        println!("{}",x);
    }
    else {
        while count < n {
            count= count+1;
            let next_value:i32 = x+y;
            x = y;
            y = next_value;
        }

    }
    return y;

    }

