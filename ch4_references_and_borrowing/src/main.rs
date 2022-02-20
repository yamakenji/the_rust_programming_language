fn main() {
    // println!("Hello, world!");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);
    let r1 = &mut s2;
    let r2 = &mut s2;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // 借用した値を変更しようと試みる -> エラー
}
