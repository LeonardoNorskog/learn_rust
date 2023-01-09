fn main() {
    // 函数
    let list = [1, 2, 3, 4];

    another_fn(list);

    let x = one_fn(11);
    println!("{}", x);
}

fn another_fn(x: [i32; 4]) {
    println!("x的第一个值是：{}", x[0]);

    // let y = 0;
    // let vx= (let b= 0);
}
fn one_fn(x: i32) -> i32 {
    return x + 4;
    // 5 + x
}
