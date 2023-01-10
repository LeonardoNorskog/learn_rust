use rand::Rng;

fn main() {
    // loop_fn();
    // while_fn();
    // for_fn();
    // rand_fn();
    // best_loops();
    best_while();
}

fn loop_fn() {
    // loop循环
    let mut count: i32 = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("result value is {}", result);
}

fn while_fn() {
    // while循环
    let mut count: i32 = 5;

    while count != 0 {
        println!("count value is {}", count);
        count -= 1;
    }
    println!("发射");
}

fn for_fn() {
    let num: [i32; 5] = [10, 20, 30, 40, 50];

    // while循环遍历数组

    let mut index = 0;

    while index < 5 {
        println!("{}", num[index]);
        index += 1;
    }

    // for 循环遍历数组

    for x in num.iter().rev() {
        println!("{}", x);
    }

    for i in (1..4).rev() {
        println!("{}", i);
    }
}

fn rand_fn() {
    let select_number = rand::thread_rng().gen_range(0..=1); //Rangegen_range(a, b)gen_range(a..b) 用.替换之前的,
    println!("{select_number}");
}

fn best_loops() {
    let mut num: i32 = 0;
    'comsider: loop {
        println!("num is {}", num);

        let mut best: i32 = 10;

        loop {
            println!("best is {}", best);
            if best == 9 {
                break;
            }
            if num == 4 {
                break 'comsider;
            }
            best -= 1;
        }
        num += 1;
    }
    println!("end is {}, hello world", num);
}

fn best_while() {
    // let tup: (i32, char, f64) = (1, '2', 1.0);

    // let (x, y, z) = (1, '2', 1.0);

    // assert_eq!(tup.0, x);
    // 数组

    let list: [i32; 4] = [0, 1, 2, 3];

    // let [a, x, y, z] = [0, 1, 2, 3];

    // assert_eq!(list[0], a);

    for i in list {
        println!("{}", i);
    }

    for i in (1..10).rev() {
        println!("{}", i);
    }
}
