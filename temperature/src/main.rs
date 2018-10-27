fn main() {
    let ftemps = [0, 32, 50, 70, 100, 212];
    let ctemps = [-10, 0, 18, 20, 30, 40, 100];

    for f in ftemps.iter () {
        println!("{} degrees F is {} degrees C", f, to_c(*f));
    }

    for c in ctemps.iter () {
        println!("{} degrees C is {} degrees F", c, to_f(*c));
    }
}

fn to_c(f: i32) -> f64 {
    let f = f as f64;
    ((f - 32.0) * 5.0 / 9.0)
}

fn to_f(c: i32) -> f64 {
    let c = c as f64;
    ((c * 9.0 / 5.0) + 32.0)
}

fn some_fun() -> () {
    println!("Hello {}", "world");
}
