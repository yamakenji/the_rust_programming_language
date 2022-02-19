fn main() {
    // println!("Hello, world!");

    // let s = String::from("hello");
    // println!("{}", s);

    // let mut s_mut = String::from("hello");
    // s_mut.push_str(",world!");
    // println!("{}", s_mut);

    // // move

    // // List4-2
    // // スタックに積まれる
    // let x = 5;
    // let y = x;

    // println!("x is {}, y is {}", x, y);

    // let s1 = String::from("hello");
    // // move
    // // let s2 = s1;
    // // clone
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // 所有権と関数
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{}", s); // ここではsは使えない！

    let x = 5;
    makes_copy(x);

    println!("{}", x); //ｘは大丈夫

    // 戻り値とスコープ
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    println!("{}", s1);
    // println!("{}", s2); s2はs3にムーブされる
    println!("{}", s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
