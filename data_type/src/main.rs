fn main() {
    //数据类型
    // let guess = "32".parse().expect("not a number");  会报错

    let guess: u32 = "32".parse().expect("not a number");
    println!("{}", guess);

    // 整数类型

    let num: u8 = 255;
    println!("{}", num);

    // 浮点类型
    let x = 2.3;
    let p: f32 = 23.4;
    println!("{},{}", x, p);
    // 布尔类型

    let o = true;
    println!("{}", o);

    // 字符类型

    let y = 'c';
    let i = '👍';
    println!("{}, {}", y, i);

    // Tuple
    let tup = (2, 2.3, 'z');
    println!("{},{},{}", tup.0, tup.1, tup.2);

    // 模式匹配
    let (q, w, e) = tup;
    println!("{},{},{}", q, w, e);
}
