fn main() {
    let result = is_divisible_by(10, 10);
    println!("{}", result);

    //let op = |x: i32, y: i32| -> i32 { x + y };

    let result1 = cacl(add, 10, 20);
    println!("{}", result1);

    println!("{:?}", (login(String::from("hello"), String::from("world")))());

    let mut name: String = String::from("hello");
    let mut age = 17;

    let closure = || {
        println!("{}", name);
        println!("{}", age);
        age = 19;
        name = String::from("world");
    };

    println!("{}", name);

    let c = test2();
    c();

    let point1 = Point1 { x:23, y: 3.14 };
    point1.m1();

    assert_eq!(2, divide(10, 5).unwrap());

    let res: Result<&str, &str> = Ok("hhhh");
    println!("{}", res.unwrap());

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", v.capacity());

    println!("result is {:?}", r#match("foobar", "foo"));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn test_borrow() {
    let x1: i32 = 34;
    let y1: Box<i32> = Box::new(45);

    {
        let (x2, y2) = (x1, y1);
    }

    println!("x1 {}", x1);
    // println!("y1 {}", y1);

    let z: [u8; 5] = [1, 2, 3, 4, 5];
}

pub fn is_divisible_by(a: u32, mut b: u32) -> bool {
    if b == 0 {
        b = 1;
    }
    a % b == 0
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> f64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn modify_x(&mut self, x: f64) {
        self.x = x;
    }

    fn modify_y(&mut self, y: f64) {
        self.y = y;
    }
}

type F = fn(i32, i32) -> i32;

fn cacl(op: F, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn cacl1<T>(op: T, a: i32, b: i32) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    op(a, b)
}

fn cacl2(op: impl Fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn login(username: String, password: String) -> impl Fn() -> String {
    move || {
        if username.eq(&"hello") && password.eq(&"world") {
            String::from("欢迎来到编程教室")
        } else {
            String::from("用户名密码错误")
        }
    }
}

fn test1() -> impl Fn() {
    let name = "test1";
    move || {
        let new_name = name;
        println!("{}", name);
    }
}

fn test2() -> impl FnOnce() {
    let name = "test2";
    move || {
        println!("{}", name);
    }
}

fn test3() -> impl Fn() {
    let name = "satori".to_string();
    let age = 17;
    let c = move || {
        // 虽然在定义闭包的时候，将name移动到了闭包内部
        // 但在使用 name 的时候，仍必须以引用的方式
        // 否则它的所有权就会转移给 new_name
        // 而 Rust 不允许这种情况出现
        let new_name = &name;
        // age 是可 Copy 的，不涉及所有权的转移
        // 因此使用 age 和 &age 均可
        let new_age = age;
    };
    // age 依然有效，它是可 Copy 的
    // 在移动的时候会拷贝一份，所以不影响
    println!("{}", age);
    // 但 name 就不行了，因为它被移动到了闭包内部
    c
}

struct Color(u8, u8, u8);

fn fn_once<F>(func: F) 
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

struct Point1<T, W> {
    x: T,
    y: W,
}

impl Point1<i32, f64> {
    fn m1(&self) {
        println!("我是 m1 方法");
    }
}

enum MyOption<A, B, C> {
    Some1(A),
    Some2(B),
    Some3(C),
}

fn test_option() {
    let x: MyOption<u8, u8, u8> = MyOption::Some3(8);
}

use std::fmt::Display;
use std::fmt::Debug;

// Display 要求变量能够以 "{}" 的方式打印
// Debug 要求变量能够以 "{:?}" 的方式打印

struct Rect<T: Display, W: Debug> {
    x: T,
    y: T,
    w: W,
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    let ret: Result<i32, &'static str>;

    if b != 0 {
        ret = Ok(a / b);
    } else {
        ret = Err("divide by zero");
    }
    ret
}

fn r#match(a: &str, b: &str) -> bool {
    a.contains(b)
}