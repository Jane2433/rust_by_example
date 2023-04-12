/// 标量类型 和 复合类型
pub fn all_type() {
    /// 1. 有符号整数：i8、i16、i32、i64、i128、isize
    /// 2. 无符号整数：u8、u16、u32、u64、u128、usize
    /// 3. 浮点：f32、f64
    /// 4. char：Unicode字符，'a', '@', '😊' ，每个都是4字节
    /// 5. bool: 只能是 true、false
    /// 6. 单元类型：只能是 () 空元组，尽管是空元组，也不认为是复合类型

    let logical: bool = true; // 类型说明
    let an_integer = 5i32; // 后缀类型说明
    let default_float = 3.0; // 默认类型
    let default_integer = 3; // 默认类型

    // 上下文自动推
    let mut inferred_type = 12; // 根据下一行的赋值推断出 i64
    inferred_type = 4294967296i64;

    // 遮蔽前面的变量
    let inferred_type = 4.0;

    // 可读性
    let long_num = 0.000_001;

    // 进制
    let two = 0b0011u32;
    let sixteen = 0x80u32;


    /// 1. 数组：[1, 2, 3]
    /// 2. 元组：(1, 2, 3)

    // 元组可以充当函数的参数和返回值
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // 解构，绑定变量
        let (a, b) = pair;

        (b, a)
    }

    // 包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 通过元组的下标来访问具体的值
    println!("tuple of tuples: {:?}", long_tuple);  // 长度超过12的元组就不能打印了
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));


    // 数组是相同类型的集合，在内存中连续：[T; length]
    // 切片由指向数据的指针和长度组成，用来借用数组的一部分：&[T]

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let ys = [0; 50];   // 默认值填充
    println!("arr occupies {} bytes", std::mem::size_of_val(&arr));

    fn analysis_slice(slice: &[i32]) {
        println!("slice's length: {}", slice.len());
    }

    analysis_slice(&arr);

    // 越界的下标会引发致命错误（panic）
    // println!("{}", arr[5]);
}