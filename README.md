# learn_rust
学习Rust
### cargo
- cargo build
    - 创建一个可执行文件位于target/debug/hello_cargo.ex
    - 在顶层目录生成一个Cargo.lock文件
    - 该文件负责追踪项目依赖的精确版本，不要手动修改
- cargo run  编译代码+执行结果
    - 如果之前编译成功且源码未修改，则直接运行二进制文件

- cargo check 检查代码，确保可以通过编译，且不会生成可执行文件
    - cargo check 比cargo build 快得多

- cargo build --release  为发布构建
    - 编译时会优化
    - 代码会运行更快，但编译时间更长
    - 会在target/release 生成可执行文件

### let,mut,use
- use 导入库 use::std::io;  表示从变量准库导入输入输出
- let 使用let声明一个变量
    - rust中所有变量默认不可变，如果想修改就使用mut
- mut 配合let声明一个可变变量
```rust
fn main() {
    let i :32 = 1;
    let mut x: u32;

    x = i;  // 这样一旦声明变量类型赋值会报错，但是不声明变量类型的话不会报错
}

- {}  格式化字符串

### rand库
- 生成一个指定范围的随机数
```rust
let select_number = rand::thread_rng().gen_range(1..100); //Rangegen_range  (a, b)gen_range(a..b) 用.替换之前的, 不包含100

let select_number = rand::thread_rng().gen_range(0..=100);  // 包含100
```

### cmd::Oedering  枚举类型
- 用于两个值的比较  有三个值  Odering::Less  Ordering::Greater  Odering::Equal

### trim() parse()
- trim()方法用于去除字符串两边的空白，包括换行符\n
- parse()方法用于将字符串解析为某种类型的数字 i32，u32，i64
    - parse()  返回的是Result枚举类型  有两个值：Err Ok

### 类型的隐式显式声明
- 隐式
```rust
let guess = 100;
```
- 显式
```rust
let guess:i32 = 100;
```
### match
- 通常用于替代expect处理出错
```rust
match {
    
}
```
### 常量
- 不可以使用mut关键字
- 使用const声明
- 可在任何作用域声明，包括全局作用域。运行期间在其作用域一直有效
- 只可绑定到常量表达式，不可绑定到函数调用结果或只能在运行时才能计算出的值
- 命名规范：常量全部大写，每个单词使用下划线分开  MAX_POINTS
- 必须声明常量类型 i32, u32, i64...
```rust
const MAX_POINTS: i32 = 100_00;
```
### shadowing
- 可以使用相同的变量名字声明新的变量， 新的变量就会shadowing之前旧的同名变量
- 使用shadowing可以改变变量的数据类型，mut不可以
```rust
let num: i32 = 100;
let num: i32 = num + 1;
```

### 数据类型
- 标量类型
    - 整数类型
        - 没有小数部分
        - 分为有符号（i32,i64）和无符号（u32,u64）
        - arch 根据系统架构  
            - iszise和usize
                - 由系统架构决定，如果是64位计算机，那就是64位 使用不多
        - 整数字面值
            - 除了byte类型，所有数值字面值都允许使用类型后缀。 例如：54u8
            - 如果不清楚哪种类型，使用Rust默认类型
            - 整数默认类型是i32 
        - 整数溢出
            - 调试模式下编译：rust会检查整数溢出，如果溢出程序在运行时会panic
            - release编译：rust不会检查可能导致panic的整数溢出。如果发生溢出，会执行"环绕"操作  
                - 例如： u8的范围是0-255，如果把u8的值设置成256，环绕之后256会变成0， 257变成1
                ```rust
                let num: u8 = 256; // 此时就会发生panic
                ```

    - 浮点类型
    ```rust
    let b: f64 = 2.3;
    ```
        - 两种基础浮点类型：
            - f32,32位,单精度
            - f64,64位,双精度
        - 默认f64且精度高

    - 布尔类型
    ```rust
    let b: bool = true;
    ```
        - true和false
        - 占一个字节
        - 符号bool
    - 字符类型
        - 使用char定义单个字符
        - 使用单引号
        - 占四字节大小
        ```rust
        b: char = 'z';
        ```

    - 数值操作与其他语言一样 加减乘除取余...

- 复合类型
    - 将多个值放在一个类型里
    - rust提供了两种基础的复合类型：元组、数组

    - Tuple
        
        - 将多个类型的多个值放在一个类型里
        - 长度是固定的，一旦声明就无法改变
        - 创建Tuple
        ```rust
        let tup：(i32, f64, char) = (2, 2.3, 'z');
        println!("{},{},{}", tup.0, tup.1, tup.2);
        ```
        - 获取Tuple的元素值
            - 可以使用模式匹配来解构一个Tuple来获取元素值
            ```rust
            // Tuple
            let tup：(i32, f64, char) = (2, 2.3, 'z');
            // 模式匹配
            let (x, y, z) = tup;
            println!("{},{},{}", x, y, z);

            let (x, y, z) = (1, 2.0, '3',);  // 解构
            ```
        - 访问Tuple元素
            - 在tuple变量使用点标记法，后接元素索引号
            ```rust
            let x: (char, i32, f64) = '1', 1, 1.1;
            println!("{}", x.0)  // 获取'1'
            println!("{}", x.1)  // 获取1
            println!("{}"， x.2)  // 获取1.1
            ```
    - 数组
        - 数组也可以将多个值放在一个类型里
        - 数组中每个元素的类型必须相同
        - 数组的长度也是固定的
        - 数组的用处
            - 可以让数据存在栈内存，而不是堆内存上，或者想保留固定数量的元素
            - 数组没有vector灵活，和数组类型，由标准库提供，长度可变，无法确定使用数组还是vector，建议用vector
        - 数组的类型
            - 表示：[类型; 长度]
        ```rust
        let v = [1, 2, 3, 4];
        let u: [f64; 4] = [1.1, 2.1, 3.1, 3.1];
        ```
        - 另一种声明数组的方法
            - 如果数组中的每个元素值都相同，那么在：中括号指定初始值，然后是一个“；”，最后是数组长度
            ```rust
            let num = [1; 3] // 意思是：生成一个有1组成的三位长度的数组
            let num = [1, 1, 1]  // 等价于上面
        - 访问数组的元素
        ```rust
        let num = [1, 2, 3, 4]
        let first = num[0]
        ```
        如果访问的索引超出了数组的范围，那么：
            - 编译时会通过 简单的会直接报错
            - 运行时会报错（panic）

        ```rust
        let num: [i32, 4] = [0, 1, 2, 4];

        let [x, y, z] = [0, 1, 2];  // 解构
        ```





- Rust在编译时必须知道变量的数据类型
- 如果类型较多，必须声明变量类型，否则会报错
```rust
let guess = "32".parse().expect("not a number");  // 会报错 type annotations needed
```
正确的应是
```rust
let guess: u32 = "32".parse().expect("not a number");
```

### 函数
- 声明函数使用fn关键字
- 遵循snake case规范： 
    - 所有字母小写，单词之间使用下划线分开
    ```rust
    fn main() {
    // 函数
    another_fn();
    }

    fn another_fn() {
        println!("hello world");
    }
    ```
- 函数的参数
    - 在函数签名里，必须声明每个参数的类型
    ```rust
    fn main() {
    // 函数
    let list = [1, 2, 3, 4];

    another_fn(list);
    }

    fn another_fn(x: [i32; 4]) {
    println!("x的第一个值是：{}", x[0]);
    }
    ```
    - 函数体中的语句与表达式
        - 函数体由一系列语句组成，可选的由一个表达式结束
        - rust是一个基于表达式的语言
        - 语句是一些动作的指令
        - 表达式计算会产生一个值
        - 函数的定义是一个语句
        - 语句不返回值，所以不能使用let将一个语句赋值给一个变量
    - 函数的返回值
        - 在-> 后面声明函数的返回类型
        - 返回值是函数体最后一个表达式的值
            - 提前返回使用return关键之，并指定一个值，默认使用最后一个表达式的值返回
        ```rust
        fn main() {

        let x = one_fn(11);
        println!("{}", x);
        }
        fn one_fn(x: i32) -> i32 {
        return x + 4;
        5 + x
        }
        ```

### 注释

### if..else..
- if表达式的条件必须是bool类型
```rust
fn main() {
    let number: i32 = 13;

    if number < 3 {
        println!("输出3");
    } else if number < 0 {
        println!("不输出");
    } else {
        println!("111");
    }
}
```

### loop while for循环
- loop循环
```rust
fn main() {
loop_fn();
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
```
- while循环
```rust
fn main() {
    while_fn();
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
- for循环
for循环通常用来遍历数组，相比于while遍历数组更安全不会出现索引错误
Range指定一个开始和结束数字， Range可以生成它们之间的数字（不含结束）
rev()方法可以反转Range
```rust
fn for_fn() {
    let num: [i32; 5] = [10, 20, 30, 40, 50];

    // while循环遍历数组

    let mut index = 0;

    while index < 5 {
        println!("{}", num[index]);
        index += 1;
    }

    // for 循环遍历数组

    for x in num.iter().rev() {  // 这个iter() 可以不写
        println!("{}", x);
    }

    for i in (1..4).rev() {
        println!("{}", i);
    }

    for i in 1..=4 {
        println!("{}", i);
    }

}
```
- 嵌套循环
```rust
fn main() {
    best_loops();
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
```


### 所有权(Rust核心)
- 所有权解决的问题
    - 跟踪代码的哪些部分正在使用heap的哪些数据
    - 最小化heap上的重复数据量
    - 清理heap上未使用的数据以避免空间不足
- 所有权规则
    - 每个值都有一个变量，这个变量是该值的所有者
    - 每个值同时只能有一个所有者
    - 当所有者超出作用域（scope）时，该值将被删除
- 变量作用域



