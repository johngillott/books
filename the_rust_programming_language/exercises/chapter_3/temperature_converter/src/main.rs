fn main() {
    let f = 194.0_f64;
    let c = 0_f64;
    println!("fahrenheit {} to celcius {}", f, f_to_c(f));
    println!("celcius {} to farhrenheit {}", c, c_to_f(c))
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0_f64) * (5.0_f64 / 9.0_f64)
}

fn c_to_f(c: f64) -> f64 {
    c * (9.0_f64 / 5.0_f64) + 32.0
}
