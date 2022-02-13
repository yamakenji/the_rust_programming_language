fn main() {
    println!("Hello, world!");

    let s = String::from("hello");
    println!("{}", s);

    let mut s_mut = String::from("hello");
    s_mut.push_str(",world!");
    println!("{}", s_mut);

    // move

    // List4-2
    // スタックに積まれる
    let x = 5;
    let y = x;

    println!("x is {}, y is {}", x, y);

    let s1 = String::from("hello");
    // move
    // let s2 = s1;
    // clone
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
