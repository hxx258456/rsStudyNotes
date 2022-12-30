fn main() {
    let _c = 'c';
    let heart_eyed_cat = '😻';

    println!("{}", heart_eyed_cat);
    // rust中char占用4字节,unicode码
    println!(
        "char占用字节大小: {}",
        std::mem::size_of_val(&heart_eyed_cat)
    );

    let t = true;
    if t {
        // rust中bool类型占用1字节
        println!("bool占用字节大小: {}", std::mem::size_of_val(&t));
    }

    // 单元类型unit type唯一的值()
    // 类似于go的struct{}{}，完全不占用内存
    // man函数的返回值就是()
    // rust中没有返回值的函数为发散函数(diverge function)
    // ()也可以用作map的值，表示不关注值只关注key
}
