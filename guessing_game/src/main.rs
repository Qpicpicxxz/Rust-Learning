// 将io模块引入当前作用域
// use关键字: 将某类型显式地将其引入作用域
// Rust设定了一些会自动导入到每个程序作用域中的标准库内容(预导入prelude), 例如println
use std::io;

// fn关键字: 声明一个新函数
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    // let apples = 5 -> Rust中，变量默认是不可变的, 添加mut关键字让变量可变
    // String是UTF-8编码的可增长文本块
    // ::表明new是String类型的一个关联函数associated function(针对某个类型实现的函数)
    let mut guess = String::new();
    // 如果开头没有引入io库, 则可以std::io::stdin
    // stdin函数返回一个std::io::Stdin的实例，代表终端标准输入句柄的类型
    // &guess默认是不可变的, 因此需要&mut guess
    // 调用了句柄上的read_line方法(无论输入什么都将其追加到字符串中)
    // 返回一个枚举类型Result(enum) -> Ok/Err
    // Result实例拥有expect方法, 如果不调用expect程序编译会出现警告
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // {}是个占位符, 用于格式化字符串里面的参数传入
    println!("You guessed: {}", guess);
}