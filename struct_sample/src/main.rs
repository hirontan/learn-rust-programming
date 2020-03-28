#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // タプル
    // let rect1 = (30, 50);

    // タプルにラベルをつける（構造体を所有するのではなく借用）
    let rect1 = Rectangle { width: 30, height: 50 };

    // 構造体を出力する
    println!("rect1 is {:?}", rect1);

    // 構造体を見やすく出力する
    println!("rect1 is {:#?}", rect1);

    // 関数シグネチャで＆を使用する
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(50);
    println!("{:#?}", sq)
}

// 関数
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// タプル利用
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// タプル with 構造体
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
