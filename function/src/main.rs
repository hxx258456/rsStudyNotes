use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
    another_function(3, 4);

    let x = plus_five(5);
    println!("{}", x);

    let y = plus_or_minus(4);
    println!("{}", y);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn another_function(i: i32, j: i32) {
    println!("the value of i is: {}", i);
    println!("the value of j is: {}", j);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        // 最后一个表达式才会认为是返回，x-5这里必须显示的指明return才行，如果不显示指明，if后面的语句还是会执行，所以if块内部需要的是一个语句，而不是表达式。
        return x - 5;
    }
    x + 5
}

// 隐式的返回()
fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

// 显示的返回()
fn clear(text: &mut String) -> () {
    *text = String::from("");
}

// 发散函数，永不返回的函数
// fn dead_end() -> ! {
//     panic("diverge function!!!!");
// }

// 发散函数永不返回的函数
fn for_ever() -> ! {
    loop {
        // ..
    }
}
