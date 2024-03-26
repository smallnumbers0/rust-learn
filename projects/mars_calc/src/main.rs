use std::io;


fn main() {
    let mut input = String::new();

    println!("Enter your weight (kg): ");
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight: f32 = calculate_weight_on_mars(weight); //mut is needed for a variable to be mutable
    println!("Weight on Mars: {}kg", mars_weight); //macros are used for metaprogramming
  
}



fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

