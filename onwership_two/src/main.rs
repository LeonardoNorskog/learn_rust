fn main() {
    let a = [1, 2, 3, 4];

    let s = &a[..];

    let s1 = &a[0..2];
    println!("{:?} {:?}", s, s1);
}
