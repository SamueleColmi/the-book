/*
temperature converter:
1) ask user for temp in Fahrenehit
2) convert string to float
3) convert temperature
4) print the result
*/

const DELTA: f64 = 32.0;
const GAMMA: f64 = 5.0 / 9.0;

fn main() {
    println!("*** Fahrenehit to Celsius converter ***");

    loop {
        let fahrenehit: f64;
        let celsius: f64;

        fahrenehit = get_user_temp();
        celsius = convert_fahrenehit_to_celsius(fahrenehit);

        println!("Conversion: {fahrenehit}°F equals to {celsius}°C");
    }
}

fn get_user_temp() -> f64 {
    let mut user_temp = String::new();

    println!("Please enter temperature in Fahrenehit");
    std::io::stdin()
        .read_line(&mut user_temp)
        .expect("Error reading temperature from user");

    convert_string_to_f64(user_temp)
}

fn convert_string_to_f64(string: String) -> f64 {
    let float = string
        .trim()
        .parse::<f64>()
        .expect("Temperature is not a number");

    float
}

fn convert_fahrenehit_to_celsius(f: f64) -> f64 {
    (f - DELTA) * GAMMA
}
