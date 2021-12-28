mod sub_a;
pub mod sub_b;

pub const MAX_POINT: u32 = 100_000;

pub fn run() {
    println!("Here is var module");
    sub_a::fun();
    sub_b::fun();

    // let 不可变 mut 申明是一个变数
    let mut x = 5;
    println!("The vakye of x is:{}", x);
    x = 6;
    println!("The vakye of x is:{}", x);

    let _i1 = 3;
    let _f1 = 0.1;

    //获取系统操作系统的位数
    print!("{}", usize::BITS);

    //p ⇨ pointer　打印一个地址
    println!("Memory address const is:{:p}", &MAX_POINT);

    let i2: i64 = 1;
    let i3: i64 = 2;

    println!("Stack address is i2 is:{:p}", &i2);
    println!("Stack address is i3  is:{:p}", &i3);

    let y = 5;
    println!("Memory address y is:{:p}", &y);
    let y = y + 1;
    println!("Memory address y is:{:p}", &y);
    let y = y * 2;
    println!("Memory address y is:{:p}", &y);
    println!("The value of y is :{}", y);
    {
        let y = 0;
        println!("The value of y is :{}", y);
    }

    let t1 = (500, 6.4, "dummy");

    let (_x, _y, _z) = t1;

    println!("the value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = -5;
    *y1_ptr = 5;
    println!("{:?}", t2);

    // index = 0
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?},{:?},{:?}", a1, a2, a1[3]);

    let s1 = "hello　こんにちは挨拶です";
    let s2 = "hello";

    println!("Stack address is s1 is:{:p}", &s1);
    println!("Stack address is s2  is:{:p}", &s2);
    println!("Memory address s1 is:{:?}", s1.as_ptr());
    println!("Memory address s2 is:{:?}", s2.as_ptr());
    println!("Len of s1 is:{:?}", s1.len());
    println!("Len of s2 is:{:?}", s2.len());

    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloword");
    println!("Stack address is s1 is:{:p}", &s1);
    println!("Stack address is s2  is:{:p}", &s2);
    println!("Capacity of s1 is :{}", s1.capacity());
    println!("Capacity of s2 is :{}", s2.capacity());
    s1.push_str("_aa");
}
