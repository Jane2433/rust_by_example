// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

pub fn all() {

    let decimal = 65.4321_f32;

    // 通过 as 关键字类型转换
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当把任何类型转为无符号类型 T 时，会不断加或减 std::T::MAX + 1 直到值位于 T 的范围内
    println!("1000 as a u16 is : {}", 1000 as u16);
    // 1000 - 256 - 256 - 256 = 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("-1 as a u8 is : {}", (-1i8) as u8);

    println!("128 as a i8 is : {}", 128 as i8);

    // 一个变量所占的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&character));

}