fn main() {
    println!("Hello, world!");

    let s = String::from("hello");
    println!("{}", s);

    let mut s_mut = String::from("hello");
    s_mut.push_str(",world!");
    println!("{}", s_mut);
}
