fn main() {
    println!("fib(8) {}", fib(8));
}

fn fib(n: isize) -> isize {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}
