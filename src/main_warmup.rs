
fn main(){
    // let a = 42;
    // let b = 100;
    // let c = 3000u16;

    // let d = 3.14;
    // let e = 2.718;

    // let f = true;

    // let g = 'R';
    // let h = '中';

    // let tup = (42, 3.14, 'r');
    // let first = tup.0;
    // let second = tup.1;

    // let arr = [1, 2, 3];
    // let first_elem = arr[0];
    // let second_elem = arr[1];

    // println!("整数：{a}, {b}, {c}");
    // println!("浮点：{d}, {e}");
    // println!("布尔：{f}");
    // println!("字符：{g}, {h}");
    // println!("元组：first = {first}, second = {second}");
    // println!("数组：first = {first_elem}, second_elem = {second_elem}");


    //控制流
    
    //if表达式

    // let x = 42;
    // let result = if x > 30 {"大于30"} else {"小于等于30"};
    // println!("{result}");


    // //loop
    // let mut count = 0;
    // let loop_result = loop {
    //     count += 1;
    //     if count == 3 {
    //         break count * 3;
    //     }
    // };
    // println!("loop返回: {loop_result}");


    // //while循环

    // let mut n = 3;
    // while n > 0 {
    //    println!("倒计时：{n}");
    //    n -= 1;
    // }
    
    // //for循环，遍历！！！
    // let arr = [10, 20, 30];
    // for elem in arr {
    //     println!("元素有这些：{elem}")
    // }


    // //范围(不熟悉)
    // for i in 0..3{
    //     println!("范围：{i}");
    // }
    // //能否在println的""里用表达式写成 {i + 1}



    // //函数调用
    // let sum = add(5, 3);
    // println!("5 + 3 = {sum}");

    // let squared = square(4);
    // println!("4的平方是：{squared}");

    // fn add(a: i32, b: i32) -> i32 {
    //     a + b
    // }

    // fn square(x: i32) -> i32 {
    //     return x * x;
    // }

    //函数可以返回{}的表达式或者直接return


    // ---- String 类型（为了演示所有权，需要堆上的数据）----
    // 字符串字面量是 &str（在栈上），String 在堆上
//     let s1 = String::from("hello");
//     let _s2 = s1;

//     //println!("{s1}");
//     println!("move 后 s2 : {_s2}");

//     // Clone(显式深拷贝)  显式深拷贝什么意思
//     let s3 = String::from("world");
//     let s4 = s3.clone();
//     println!("clone: s3 = {s3}, s4 = {s4}");

//     //Copy类型，（栈上数据自动拷贝）
//     let x = 5;
//     let y = x;
//     println!("Copy: x = {x}, y = {y}");

//     //函数传参也move
//     let s5 = String::from("take");
//     take_ownership(s5);
//     //println!("{s5}");

//     //函数所有值可转移所有权
//     let s6 = give_ownership();
//     println!("函数返回：{s6}");

// }

// fn take_ownership(s: String) {
//     println!("拿走了：{s}");
// }

// fn give_ownership() -> String {
//     let s = String::from("given");
//     s

    // //不可变引用：
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("{s1} 的长度是： {len}");

    // //可变引用：
    // let mut s2 = String::from("hello");
    // change(&mut s2);
    // println!("s2 变为了： {s2}");

    // //可变独占
    // let mut s3 = String::from("world"); //这里的s3不需要用mut修饰吧？
    // let r1 = &s3;
    // let r2 = &s3;
    // println!("{r1}, {r2}");


    // //let r3 = &mut s3;
    // //println!("{r1}  {r2}");

    // //数据竞争
    // let mut s4 = String::from("test");
    // {
    //     let r = &mut s4;
    // }
    // //这里r在这里结束的原因是因为作用域受限，所以可以新建可变引用是吗
    // let r2 = &mut s4;
    // println!("{r2}");


    //String Slice
//     let s = String::from("hello world");
//     let hello = &s[0..5];
//     let world = &s[6..11];
//     println!("{hello}, {world}");


//     //..= 包含右边界
//     let hello2 = &s[..=4];
//     let whole = &s[..];
//     println!("{hello2}, {whole}");


//     //函数参数首选&str，而非&String
//     // &String 只能传String &str可以传 String slice和字面量
//     let my_str = "hello"; //这是一个str吧
//     print_it(my_str); //这是本身
//     print_it(&s[0..5]); // 这是slice
//     print_it(&s);  //这是不可变引用String
//     //上边这部分需要解释一下

//     //数组slice

//     let arr = [1, 2, 3, 4, 5];
//     let slice = &arr[1..4];
//     println!("数组slice： {slice:?}"); //解释一下调试输出

//     //返回第一个单词
//     let word = first_word(&s);
//     println!("第一个单词：{word}");

// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();//为何这样处理
//     for (i, &bytes) in bytes.iter().enumerate() {//这种遍历方式第一次见
//         if bytes == b' '{  //这里b' '就代表空格吗
//             return &s[0..i];
//         }
    
//     }
//     &s[..]
// }

// fn print_it(s: &str) {
//     println!("{s}");
// }






// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(s: &mut String) {
//     s.push_str("world");
// }


// let file = FileItem {
//     name: String::from("main.rs"),
//     size: 1024,
//     is_dir: false,
// };

// println!("文件名：{}， 大小：{}", file.name, file.size);


// //可变实例
// let mut file2 = FileItem {
//     name: String::from("temp"),
//     ..file  //这里要记住
// };

// file2.size = 2048;
// println!("{}, {} 字节", file2.name, file2.size);

// //元组结构体

// let _red = Color(255, 0, 0);//为什么命名要加_
// println!("R: {}, G: {}, B: {}",_red.0, _red.1, _red.2);

// //dbg 宏
// let val = dbg!(42 * 3);
// dbg!(&file); //这里的作用

// println!("val = {val}");




// }

// #[derive(Debug)]
// #[allow(dead_code)]
// struct FileItem {
//     name: String,
//     size: u64,
//     is_dir: bool,
// }

// #[derive(Debug)]
// struct Color(u8, u8, u8);


    // let file = FileItem::new("main.rs", 1024);
    // file.print_info();

    // let mut file2 = FileItem::new("temp.txt", 512);
    // file2.resize(2048);
    // file2.print_info();

    // let size = FileItem::max_size();
    // println!("单个文件最大支持: {} 字节", size);

    // let f = FileItem {
    //     name: String::from("readme.md"),
    //     size: 100,
    // }; //使用关联函数与这种形式都能对结构体初始化
    // f.print_info();

    let light = TrafficLight::Red;

    let action = match light {
        TrafficLight::Red => "停",
        TrafficLight::Yellow => "等",
        TrafficLight::Green => "行",

    };

    println!("灯的状态: {}", action);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let addr_desc = match home {
        IpAddr::V4(a,b ,c ,d ) => format!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(s) => format!("IPv6: {}", s),
    };

    println!("{}", addr_desc);

    let msg = Some(String::from("你好"));

    if let Some(content) = msg {
        println!("有消息：{}", content);
    } else {
        println!("没有消息");
    }

    let fruit = Fruit::Orange;

    match fruit {
        Fruit::Apple => {
            println!("苹果")
        },
        Fruit::Orange => {
            println!("橘子")
        },
    };

    if let Fruit::Orange = fruit {
        println!("是橘子");
    } else {
        println!("不是橘子")
    }


}


#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),

}

#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
}

// #[derive(Debug)]
// struct FileItem{
//     name: String,
//     size: u64,
// }

// impl FileItem {
//    //关联函数
//    fn new(name: &str, size: u64) -> Self {
//     FileItem { name: String::from(name), size }
//    }//这个函数返回Self是因为类似于构造函数，调用过程中生成一个自己

//    //只读方法,所以传入不可变引用的自己
//    fn print_info(&self) {
//     println!("{} -- {}字节", self.name, self.size);
//    }

//    //&mut self 修改自身
//    fn resize(&mut self, new_size: u64){
//     self.size = new_size;
//    }


//    //为什么会出现使用.调用函数，也会出现使用::调用函数，有什么区别
//    //关联函数，不传self，类似静态方法
//    fn max_size() -> u64 {
//     1024 * 1024 *1024 
//    }
// }
