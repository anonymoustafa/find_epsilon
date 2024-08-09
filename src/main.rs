fn main() {
    let mut count = 0;
    let mut epsilon: f64 = 1.0;

    loop {
        let new_epsilon = epsilon / 2.0;

        // Check if adding new_epsilon to 1.0 results in a value greater than 1.0
        if 1.0 + new_epsilon == 1.0 {
            println!("Breaking out of the loop!");
            println!("Total iterations: {}", count);
            println!("This computer's smallest epsilon is: {}", epsilon);
            break;
        }

        epsilon = new_epsilon;
        count += 1;
    }
}

