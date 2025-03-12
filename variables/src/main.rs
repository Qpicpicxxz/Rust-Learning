fn main() {
    // mut(Abbr.) -> mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // 常量: 总是不可变且必须注明值的类型
    // 编译器能够在编译时计算一组有限的操作
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let y = 5;
    // 这里是通过let创建了一个新的变量
    let y = y + 1;
    {
        let y = y * 2;  // 作用域只在大括号里面, 因此最下面的y还是大阔号外面的y
        println!("{y}");
    }
    println!("{y}");

    // mut 与隐藏的另一个区别是
    // 当再次使用 let 时，实际上创建了一个新变量
    // 我们可以改变值的类型，并且复用这个名字
    let spaces = "       ";
    let spaces = spaces.len();
    println!("{spaces}");

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // 复合类型compound types - 元组tuple和数组array
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, z) = tup;  // destructuring - 解构
    println!("{z}");
    println!("{}", tup.1);  // 访问元组

    // 数组中每个元素类型必须相同, Rust中的数组长度是固定的
    // 这个是在栈而不是在堆上分配数据
    let _a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5]; // 与let a = [3, 3, 3, 3, 3]效果相同

    print_labeled_measurement(5, 'h');

    // 这个值作为 let 语句的一部分被绑定到 y 上, {}里面是一个表达式
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    println!("The value of x is: {}", five());

    println!("The value of y + 1 is: {}", plus_one(y));

    let number = 3;
    // if表达式中与条件关联的代码块有时被叫做arms(分支)
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // let number_bool: bool = number.parse();

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 返回值不命名但是要在->后声明它的类型
fn five() -> i32 {
    let x = 5;
    return x;
}

// 没有;表示一个表达式
// 表明类型的格式 - 变量: 类型
fn plus_one(x: i32) -> i32 {
    // return x + 1;
    x + 1
}