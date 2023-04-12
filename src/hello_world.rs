use std::fmt;
use std::fmt::{Formatter, write};
use std::io::Lines;

/// 若想使用 std::fmt 打印，要求至少实现一个可打印的 traits

/// fmt::Debug 使用简单，可通过 derive 自动推导构建。使用 {:?} 标记
pub fn debug() {
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} is a num", 12);
    println!("{1:?} {0:?} {name:?}", "b", "a", name = "zzl");
    println!("Structure: {:?}", Structure(7));
    println!("Deep: {:?}", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "zzl";
    let age = 18;
    println!("Person: {:#?}", Person { name, age });
}

/// fmt::Display 需要手动实现，从而自己控制输出内容，使用 {} 标记
pub fn display() {
    // 导入模块，使 fmt::Display 可用
    use std::fmt;

    #[derive(Debug)]
    struct Complex {
        real: f32,
        imag: f32,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

/// write! 可以一个一个的写出到管道，通过 ? 来传递错误
pub fn throw() {
    use std::fmt;

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;

            for (index, num) in self.0.iter().enumerate() {
                if index != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", index, num)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

/// 格式化 format! 使用同 println!
pub fn format() {
    let foo: u32 = 3735928559;

    println!("{}", foo);
    println!("{:x}", foo);  // 16进制
    println!("{:X}", foo);  // 16进制
    println!("{:o}", foo);  // 8 进制

    println!("{:.3}", 53.347778); // 保留3位小数
    println!("{:03}", 5); // 补0
}
