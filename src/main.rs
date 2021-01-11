fn main() {
    let mut mars_weight = calculate_mars_weight(90.5);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}g", mars_weight);

}

fn calculate_mars_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}