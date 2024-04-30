// fn f<T>(ch: char, num1: T, num2: T) -> T {
//     if ch == 'a' {
//         num1
//     } else {
//         num2
//     }
// }

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Division by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

// fn show_divide(num: f64, den: f64) {
//     match divide(num, den) {
//         Ok(x) => println!("Result = {}", x),
//         Err(e) => println!("Error: {}", e),
//     }
// }

fn main() {
    // let a = f('a', 37, 41);
    // let b = f('b', 37.2, 41.1);
    // println!("a = {}, b = {}", a, b);

    // let mut v = vec![11, 22, 33];
    // for _ in 0..5 {
    //     let item = v.pop();
    //     match item {
    //         Some(x) => println!("x = {}", x),
    //         None => println!("None"),
    //     }
    // }

    // show_divide(8., 2.);
    // show_divide(8., 0.);

    let r1 = divide(8., 2.);
    let r2 = divide(8., 0.);
    println!("{} {}", r1.is_ok(), r1.is_err());
    println!("{} {}", r2.is_ok(), r2.is_err());
    println!("{}", r1.unwrap());
    println!("{}", r2.unwrap());
}