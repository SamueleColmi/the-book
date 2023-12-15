fn main() {
    let cond = true;

    let x = if cond { 5 } else { 10 };

    if x < 7 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }
}
