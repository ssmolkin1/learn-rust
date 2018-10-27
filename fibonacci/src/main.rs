fn main() {
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in numbers.iter() {
        println!("The {}th Fibonnaci number is: {}.", i, fib(*i));
    }
}

fn fib(n: u32) -> u32 {
    let mut xm2: u32 = 0;
    let mut xm1: u32 = 1; 
    let mut x: u32 = 1;

    if n <= 1 {
        return n;
    } 

    for i in 2..n {
        xm2 = xm1;
        xm1 = x;
        x = xm1 + xm2;
    }

    x
}
