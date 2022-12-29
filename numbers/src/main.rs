//! # numbers
//!
//! 基础类型示例
//! 数值类型：有符号整数(i8, i16, i32, i64, isize)、无符号整数(u8, u16, u32, u64, usize)、浮点数(f32, f64)、以及有理数、复数

fn main() {
    // rust编译器可以自动进行简单的类型推导
    let _guess: i32 = "42".parse::<i32>().expect("Not a number!");

    let dec_int = 18;
    let hex_int: i16 = 0xff;
    let oct_int = 0o755;
    let bin_int = 0b1111_0000;
    let byte = b'A';

    println!("dec_int: {:?}", dec_int);
    println!("hex_int: {:?}", hex_int);

    println!("oct_int: {:?}", oct_int);
    println!("bin_int: {:?}", bin_int);
    println!("byte: {:?}", byte);

    // 需要注意rust整形存在溢出问题，以u8为例，debug模式下256会变成0，257会变成1，以此类推，程序不会panic，需要注意
    let a: u8 = 0xff;
    let b = a.wrapping_add(20);
    println!("b: {:?}", b);

    // (0.1_f64 + 0.2 - 0.3).abs() < 0.000001;
    // rust浮点数同样存在精度问题，不建议直接比较
    // assert!(0.1 + 0.2 == 0.3);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!(" 0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!(" 0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f32)");
    println!(" 0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!(" 0.3: {:x}", (xyz.2).to_bits());
    println!();

    // 对于数学上未定义的结果，rust浮点数使用NaN来处理这些情况:
    let x = (-42.1_f32).sqrt();
    println!("{:?}", x);
    // 所有跟NaN的交互操作都会返回NaN,NaN不能比较会报错
    // assert_eq!(x, x);
    // 可以使用is_nan()判断
    if x.is_nan() {
        println!("未定义的数学行为!");
    }


    
}
