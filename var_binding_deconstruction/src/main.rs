//! #var_binding_deconstruction
//!
//! 提供变量绑定和解构示例
//! rust支持手动设置变量可变性mut

fn main() {
    // 字符串对象hello world绑定变量a
    let _a = "hello world";

    // rust变量默认不可变,使用mut声明变量可变
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 可以使用下划线开头忽略未使用的变量
    let _x = 5;

    // rust支持变量解构
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    // 解构式赋值
    let (a1, b1, c1, d1, e1);

    (a1, b1) = (1, 2);
    [c1, .., d1, _] = [1, 2, 3, 4, 5];
    Struct { e1, .. } = Struct { e1: 5 };

    assert_eq!([1, 2, 3, 4, 5], [a1, b1, c1, d1, e1]);

    // 常量
    const MAX_POINTS: u32 = 100_0000;

    // 变量遮蔽shadowing;
    // rust允许声明相同变量名，在后面声明的变量会遮蔽掉前面声明的
    let mut spaces = "   ";
    spaces = spaces.len();
    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();
}

struct Struct {
    e1: i32,
}
