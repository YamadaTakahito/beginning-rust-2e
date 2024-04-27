fn main() {
    let x = ["English", "This", "sentence", "a", "in", "is"];
    println!("{} {} {} {} {} {}", x[5], x[4], x[3], x[2], x[1], x[0]);
    println!("{:?}", x);
    #[allow(unused_variables)]
        let z = 3;

    let mut fib = [1; 12];
    for i in 2..fib.len() {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    for i in 0..fib.len() {
        println!("{}", fib[i]);
    }

    let mut x = vec!["this", "is"];
    println!("{} {}. Length {}", x[0], x[1], x.len());
    x.push("a");
    println!("{} {} {}. Length {}", x[0], x[1], x[2], x.len());
}