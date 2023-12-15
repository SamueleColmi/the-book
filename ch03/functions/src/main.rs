fn main() {
    let a;
    let b;

    a = 10;
    b = increment(a);
    display(a, 'a');
    display(b, 'b');

    let c = {
        a + b;
    };
    display(c, 'c');
}

fn display(x: i32, name: char) {
    println!("{name}: {x}");
}

fn increment(x: i32) -> i32 {
    x + 1
}
