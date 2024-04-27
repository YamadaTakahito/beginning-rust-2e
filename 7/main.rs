fn main() {
    #[allow(dead_code)]
    enum Continent {
        Africa,
        Antarctica,
        Asia,
        Europe,
        NorthAmerica,
        Oceania,
        SouthAmerica,
    }
    let mut contain = Continent::Asia;
    match contain {
        Continent::Africa => println!("Africa"),
        Continent::Antarctica => println!("Antarctica"),
        Continent::Asia => {
            let a = 7;
            println!("{}", a);
        }
        Continent::Europe => println!("Europe"),
        Continent::NorthAmerica => println!("North America"),
        Continent::Oceania => println!("Oceania"),
        Continent::SouthAmerica => println!("South America"),
    }

    #[allow(dead_code)]
    enum Result {
        Success(u8),
        Failure(u16, char),
        Uncertainty,
    }
    let outcome = Result::Failure(20, 'X');
    match outcome {
        Result::Success(0) => println!("Result: 0"),
        Result::Success(1) => println!("Result: 1"),
        Result::Success(__) => println!("Result: something else"),
        Result::Failure(10, 'X') => println!("Error: 10 X"),
        Result::Failure(10, _) => println!("Error: 10"),
        Result::Failure(code, module) => println!("Error: {} {}", code, module),
        Result::Uncertainty => println!("Unknown result"),
    }

    #[allow(dead_code)]
    enum E {
        Case1(u32),
        Case2(char),
        Case3(i64, bool),
    }
    let v = E::Case2('a');
    if let E::Case3(a, b) = v {
        println!("{} {}", a, b);
    }
}