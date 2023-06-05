// Rust 3
// Receive a numerical range and tell which numbers inside are prime.

fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {

    let mut input = String::new();

    let lower: i32 = {
        println!("Enter a lower bound: ");
        std::io::stdin().read_line(&mut input).expect("Error while reading your input.");
        input.trim().parse().expect("Error while parsing your input.")
    };

    input.clear();

    let upper: i32 = {
        println!("Enter an upper bound: ");
        std::io::stdin().read_line(&mut input).expect("Error while reading your input.");
        input.trim().parse().expect("Error while parsing your input.")
    };

    for i in lower..upper {
        if is_prime(i) {
            println!("{i} is prime.");
        }
    }

}