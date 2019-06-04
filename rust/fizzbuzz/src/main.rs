fn main() {
    for i in 1..=20 {
        /*
        if      i % 15 == 0 { println!("{}", "FizzBuzz"); }
        else if i %  5 == 0 { println!("{}", "Buzz"); }
        else if i %  3 == 0 { println!("{}", "Fizz"); }
        else { println!("{}", i); }
        */
        match i {
            _ if i % 15 == 0 => { println!("{}", "FizzBuzz"); }
            _ if i %  5 == 0 => { println!("{}", "Buzz"); }
            _ if i %  3 == 0 => { println!("{}", "Fizz"); }
            _ => { println!("{}", i); }
        }
    }
}
