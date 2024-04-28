fn main() {
    let data = (10000, 183.19, 'Q');
    let copy_of_data = data;
    println!("{} {} {}", copy_of_data.0, copy_of_data.1, copy_of_data.2);

    struct SomeData {
        integer: i32,
        fractional: f64,
        character: char,
        five_bytes: [u8; 5],
    }
    let data = SomeData {
        integer: 10000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [0, 1, 2, 3, 4],
    };
    println!("{} {} {} {:?}", data.integer, data.fractional, data.character, data.five_bytes)
}