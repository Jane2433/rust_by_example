pub fn structure() {
    // 单元结构体
    struct Unit;

    // 元组结构体，具有名称的元组
    struct Pair(i32, f32);

    // 经典结构体
    struct Point {
        x: f32,
        y: f32,
    }

    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let point1 = Point { x: 0.3, y: 0.4 };

    let x = 0.3;
    let y = 0.4;
    let point2 = Point { x, y };

    println!("point corr: {} - {}", point1.x, point2.y);

    // 使用 point2 的字段数据
    let bottom_right = Point { x: 0.5, ..point2 };

    // 解构
    let Point { x: x1, y: y1 } = bottom_right;
    print!("{} - {}", x1, y1);
}

pub fn enum_type() {
    // 从数个取值中任选一的数据类型
    enum WebEvent {
        // 单元结构体
        PageLoad,
        // 元组结构体
        KeyPress(char),
        // 结构体
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("pageLoad"),
            WebEvent::KeyPress(c) => println!("keyPress: {}", c),
            WebEvent::Click { x: a, y } => println!("click: {} {}", a, y),
        }
    }

    /// 类型别名
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        ADD,
        PLUS,
    }

    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
    let a = Operations::ADD;

    // 最常见的情况就是在 impl 块中使用 Self 别名
    impl Operations {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::ADD => x + y,
                Self::PLUS => x - y
            }
        }
    }

    /// 使用 use 引用，便可不使用完整路径了
    use WebEvent::{Click, KeyPress, PageLoad};
    use VeryVerboseEnumOfThingsToDoWithNumbers::*;
    // use Operations::*;       // 别名不让用

    let key_press = KeyPress('a');
    let add = ADD;

    match add {
        ADD => println!("add"),
        PLUS => println!("plus"),
    }

    // C 风格
    enum Number {
        ZERO,
        ONE,
        TWO,
    }

    println!("ZERO is {}", Number::ZERO as i32);
    println!("ONE is {}", Number::ONE as i32);
    println!("TWO is {}", Number::TWO as i32);

    // 显示赋值
    enum Color {
        RED = 0xff0000,
        BLUE = 0x00ff00,
        GREEN = 0x0000ff,
    }

    println!("RED is {}", Color::RED as i32);
    println!("BLUE is {}", Color::BLUE as i32);
    println!("GREEN is {}", Color::GREEN as i32);

    /// 链表
    use List::*;

    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        // 创建空实例
        fn new() -> List {
            Nil
        }

        // 头部插入
        fn prepend(self, ele: u32) -> List {
            Cons(ele, Box::new(self))
        }

        // 计算长度
        fn len(&self) -> u32 {
            // 匹配具体的类型 T 好于匹配引用 &T
            match *self {
                Cons(_, ref tail) => 1 + tail.len(),
                // 递归的基准情形（base case）
                Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match *self {
                Cons(ele, ref tail) => format!("{}, {}", ele, tail.stringify()),
                Nil => format!("Nil"),
            }
        }
    }

    let mut list = List::new();
    list = list.prepend(3);
    list = list.prepend(2);
    list = list.prepend(1);

    println!("length: {}", list.len());
    println!("to_string: {}", list.stringify());
}

/// 常量需要显示的声明类型
pub fn constant_type() {

    // const 不可改变的值，常用
    const HOLD: i32 = 10;

    // static：具有 static 生命周期的，可以是可变的（mut）
    static LANGUAGE: &'static str = "Rust";
}