fn main() {
    let s = String::from("hello");

    // let s1 = s;
    let s1 = s.clone();

    print!("{s}, {s1}");
}
