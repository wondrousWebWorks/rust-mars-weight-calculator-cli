fn main() {
    println!("{}", calculate_mars_weight(90.5));

}

fn calculate_mars_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}