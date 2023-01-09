// const MAX_POINTS: u32 = 101_000;

fn main() {
    println!("Hello, world!");

    // 当前x值是不可变的
    let x = 5;
    println!("x is {}", x);

    // x = 6;   报错，不可两次赋值

    // 可变变量
    let mut d = 100;

    println!("d is {}", d);

    d = 32;

    println!("d is {}", d);

    println!("this is emji 🔢");

    // 常量
    // const MAX_POINT: i32 = 100_000;

    // shadowing
    let z = 100;
    let z = z + 1;
    let z = z * 2;
    println!("z is {}", z);

    let a = "   ";
    let a = a.len();

    println!("a is {}", a);

    // let mut name = "";
    // name = 123;  会报错，类型不一样


}
