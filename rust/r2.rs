// Rust 2
// Receive 3 numbers from the user to represent a quadratic equation. Calculate its real solutions (check) and print them.
// Make sure the obtained values are consistent.

fn main() {

    let mut input = String::new();

    println!("Please, enter A: ");
    let a = loop {
        std::io::stdin().read_line(&mut input).expect("Error");
        let a: f64 = input.trim().parse().expect("Error");
        input.clear();

        if a == 0.0 {
            println!("'A' must be different than zero! Try again");
            continue;
        } else {
            break a;
        }

    };

    let b = {
        println!("Please, enter B: ");
        std::io::stdin().read_line(&mut input).expect("Error");
        let b: f64 = input.trim().parse().expect("Error");
        input.clear();
        b
    };

    let c = {
        println!("Please, enter C: ");
        std::io::stdin().read_line(&mut input).expect("Error");
        let c: f64 = input.trim().parse().expect("Error");
        input.clear();
        c
    };

    let discriminant = b.powi(2) - (4.0 * a * c);

    if discriminant < 0.0 {
        println!("There are no real solutions.");
    } else {

        let d_sqrt = discriminant.sqrt();

        let x1 = (-b + d_sqrt) / (2.0 * a);
        let x2 = (-b - d_sqrt) / (2.0 * a);

        println!("x' = {x1}, x\" = {x2}");

    }

}