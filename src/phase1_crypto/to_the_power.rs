fn to_the_power_of(integer: u32, power: u32) -> u32 {
    // Store new value each time after multiplying it with integer.
    // Make sure the entire power cycle is done.
    let mut result = 1;

    for i in 0..power {
        result *= integer;
    }

    result
}

fn main() {
    let test = to_the_power_of(2, 8);

    println!("{test:?}");
}