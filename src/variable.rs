pub fn all() {

    // 变量绑定默认不可变
    let _immutable_binding = 1;     // 开头下划线消除编译器警告
    let mut mutable_binding = 1;

    mutable_binding += 1;

    println!("{}", _immutable_binding);
    println!("{}", mutable_binding);

    // 变量遮蔽
    let person = "zzl";
    let person = 18;
}