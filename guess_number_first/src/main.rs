// 猜测随机数
use rand::Rng; // 生成随机数
use std::cmp::Ordering;
use std::io; //  用于输入输出 // 用于比较

// fn main() {
//     println!("猜数游戏！！！");

//     println!("猜测一个数：");

//     let select_number = rand::thread_rng().gen_range(1..101);

//     println!("神秘数字是：{}", select_number);

//     let mut guess = String::new();

//     io::stdin().read_line(&mut guess).expect("无法读取行");

//     println!("你猜测的数是：{}", guess);
// }
fn main() {
    println!("猜数字游戏！！！");

    // 生成一个0，100的随机数
    let select_number = rand::thread_rng().gen_range(0..101); // i32,i64,u32  不手动声明默认是i32

    // println!("神秘数字是：{}", select_number);

    loop {
        println!("请输入一个数字");

        let mut guess = String::new();

        // 读取输入的值

        io::stdin().read_line(&mut guess).expect("无法读取行");

        // 类型转换  shadow
        // let guess: u32 = guess.trim().parse().expect("please type a number");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type a number");
                continue;
            }
        };
        // 比较guess和select_number

        match guess.cmp(&select_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        }
    }
}
