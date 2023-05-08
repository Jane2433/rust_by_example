pub fn all() {

    // From trait 允许一种类型定义 “通过另一种类型生成自己”，在标准库中有无数的实现

    // 例如 &str String
    let my_str = "zzl";
    let my_string = String::from(my_str);

    // 为自己的类型定义转换
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Number { value }
        }
    }

    let num = Number::from(12);
    println!("{:?}", num);

    // 如果你的类型实现了 From，那么他同时获得了 Into
    let int = 5;
    let num: Number = int.into();
    println!("{:?}", num);


    // TryFrom、TryInto 用于易出错的转换，他的返回类型是 Result
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));


    // 要把任何类型转成 String，只需要实现 ToString trait
    // 然而实际上，应该实现 fmt::Display tarit，他自动提供 ToString
    struct Circle {
        radius: i32,
    }

    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());


    // 只要目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i64>().unwrap();

}