enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    // let _a1: [u8; 9000000] = [1; 9000000];

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);
    println!("Stack address of v3 is: {:p}", &v3);

    println!("Heap memory address is: {:?}", v1.as_ptr());
    println!("LEN of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());

    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);

    let t1: (i64, String) = (10, String::from("HELLO"));
    println!("Stack address of tuple data  is: {:p}", &t1);
    println!("{:?}", t1);

    let mut b1 = Box::new(t1);
    (*b1).1 += "world";

    println!("{:?}---{:?}", b1.0, b1.1);
    println!("{:?}", b1);
}
