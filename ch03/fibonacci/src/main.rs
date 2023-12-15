/*
Fibonacci:
1- ask user for an integer
2- computes fibonacci number
3- print result
*/

fn main() {
    println!("*** Fibonacci Generator ***");

    loop {
        let n: u64;
        let fibonacci: u64;

        n = get_user_input();
        fibonacci = compute_fibonacci(n);
        println!("Fibonacci of {n} is {fibonacci}");
    }
}

fn get_user_input() -> u64 {
    let mut n = String::new();

    println!("Please enter an integer");
    std::io::stdin()
        .read_line(&mut n)
        .expect("Error reading number");

    str_to_u64(n)
}

fn str_to_u64(string: String) -> u64 {
    let n: u64;

    n = string
        .trim()
        .parse::<u64>()
        .expect("Input is not an integer");

    n
}

fn compute_fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    return compute_fibonacci(n - 1) + compute_fibonacci(n - 2);
}
