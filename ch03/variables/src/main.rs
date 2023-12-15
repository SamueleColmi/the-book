fn main() {
    let tup1 = ("gennaio", 31);
    let tup2 = ("febbraio", 28);
    let tup3 = ("marzo", 31);
    let tup4 = ("aprile", 30);
    let tup5 = ("maggio", 31);
    let tup6 = ("giugno", 30);
    let tup7 = ("luglio", 31);
    let tup8 = ("agosto", 31);
    let tup9 = ("settembre", 30);
    let tup10 = ("ottobre", 31);
    let tup11 = ("novembre", 30);
    let tup12 = ("dicembre", 31);

    let arr = [
        tup1, tup2, tup3, tup4, tup5, tup6, tup7, tup8, tup9, tup10, tup11, tup12,
    ];

    println!("Insert a month");
    let mut month = String::new();
    std::io::stdin().read_line(&mut month).expect("Not a month");
    println!("You selected: {month}");

    let month = month.trim().parse::<usize>().expect("not a number");
    println!("{} has {} days", arr[month - 1].0, arr[month - 1].1);
}
