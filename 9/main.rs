fn main() {
    // let x = 4.;
    // let res = print_double(x);
    // println!("{}", x);
    // println!("res: {}", res);
    //
    // let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    // arr = double(arr);
    // println!("{:?}", arr);

    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double2(&mut arr);
    println!("{:?}", arr);
}

fn print_double(mut x: f64) -> f64 {
    x *= 2.0;
    println!("{}", x);
    x
}

fn double(mut a: [i32; 10]) -> [i32; 10] {
    for i in 0..10 {
        a[i] *= 2;
    }
    a
}

fn double2(a: &mut [i32; 10]) {
    for i in 0..10 {
        a[i] *= 2;
    }
}