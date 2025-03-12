// 将io模块引入当前作用域
// use关键字: 将某类型显式地将其引入作用域
// Rust设定了一些会自动导入到每个程序作用域中的标准库内容(预导入prelude), 例如println
use std::io;
// 在Rust中, trait定义的方法默认不在作用域中(Rust设计哲学 - 避免隐式的行为)
// trait定义了一组方法签名
use rand::Rng;
// Ordering也是一个枚举, 成员是Less/Greater/Equal
use std::cmp::Ordering;

// fn关键字: 声明一个新函数
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    // let apples = 5 -> Rust中，变量默认是不可变的, 添加mut关键字让变量可变
    // String是UTF-8编码的可增长文本块
    // ::表明new是String类型的一个关联函数associated function(针对某个类型实现的函数)
    // let mut guess = String::new();

    // 如果开头没有引入io库, 则可以std::io::stdin
    // stdin函数返回一个std::io::Stdin的实例，代表终端标准输入句柄的类型
    // &guess默认是不可变的(immutable), 因此需要&mut guess
    // 调用了句柄上的read_line方法(无论输入什么都将其追加到字符串中)
    // 返回一个枚举类型Result(enum) -> Ok/Err
    // Result实例拥有expect方法, 如果不调用expect程序编译会出现警告
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // {}是个占位符, 用于格式化字符串里面的参数传入
    // println!("You guessed: {}", guess);

    // Rust允许用一个新值来隐藏shadowing之前的值, 常用语需要转换值类型的场景
    // String 实例的 trim 方法会去除字符串开头和结尾的空白字符
    // guess 后面的冒号（:）告诉 Rust 我们指定了变量的类型
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // secret_number是一个数字类型
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // guess.trim()有点类似pub fn trim(&self) -> &str
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}