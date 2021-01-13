use std::io::{self, Write};

fn main() {
    print!("Enter weight in kilograms: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight = calculate_mars_weight(weight);
    println!("Weight on Mars: {}g", mars_weight);

}

fn calculate_mars_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}