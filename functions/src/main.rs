fn five() -> i32 {
    5
}


fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let _x = 5;
    let _y = {
        let _x = 3;
        _x + 1
    };

    println!("The value of _y is: {}", _y);

    let _x = five();
    println!("The value of _x is: {}", _x);

    let _x = plus_one(5);
    println!("The value of _x is: {}", _x);

}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}.", x);
    println!("The value of y is: {}.", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}