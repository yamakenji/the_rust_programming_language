fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    let x2 = 5;
    let x2 = x2 + 1;
    let x2 = x2 * 2;
    println!("The value of x2 is: {}", x2);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let x = 2.0;
    let y: f32 = 3.0;

    println!("{}, {}", x, y);

    // 数値演算
    // 足し算
    let sum = 5 + 10;
    // 引き算
    let difference = 95.5 - 4.3;
    //掛け算
    let product = 4 * 30;
    //割り算
    let quotient = 56.7 / 32.2;
    // あまり
    let remainder = 43 % 5;

    println!("数値演算");
    println!("{}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);

    // タプル
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("タプル");
    // println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("x is: {}, y is: {}, z is: {}", five_hundred, six_point_four, one);

    // 配列
    let arr = [1, 2, 3, 4, 5];

    let arr_first = arr[0];
    let arr_second = arr[1];

    println!("The value of element is: first {}, second {}", arr_first, arr_second);

    // let index = 10;
    //
    // let element = arr[index];
    //
    // println!("The value of element is: {}", element);

}
