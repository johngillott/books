fn main() {
    println!("Hello, world!");

    let y = {
        let x = 13;
        x + 1
    };

    another_function(forty_one(), y);
}

fn forty_one() -> i32 {
    41
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
