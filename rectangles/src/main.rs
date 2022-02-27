#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // println!("Hello, world!");

    // let width1 = 30;
    // let height1 = 50;

    // タプルをつかう
    // let rect1 = (30, 50);

    // 構造体をつかう
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        area(&rect1)
    );
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
