fn f_to_c(fahrenheit_temps: &[i32]) -> Vec<i32> {
    let mut celsius_temps = Vec::new();

    for &fahrenheit in fahrenheit_temps {
        let celsius = ((fahrenheit - 32) * 5) / 9;
        celsius_temps.push(celsius);
    }

    celsius_temps
}

fn main() {
    let fahrenheit_temps = [32, 212, 68, 104, 50];
    let celsius_temps = f_to_c(&fahrenheit_temps);

    println!("Fahrenheit Temps: {:?}", &fahrenheit_temps);
    println!("Celsius Temps: {:?}", &celsius_temps);
}
