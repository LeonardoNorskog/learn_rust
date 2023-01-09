fn main() {
    let number: i32 = 13;

    if number < 3 {
        println!("输出3");
    } else {
        println!("不输出");
    }

    let x: i32 = 45;

    if x % 1 == 0 {
        println!("1");
    } else if x % 5 == 0 {
        println!("2");
    } else if x % 9 == 0 {
        println!("3");
    } else {
        println!("4");
    }
}

// fn test() {
//     let (a, mut b): (bool,bool) = (true, false);
//     // a = true,不可变; b = false，可变
//     println!("a = {:?}, b = {:?}", a, b);

//     b = true;
//     assert_eq!(a, b);
//     assert_eq!(2,2);
// }
