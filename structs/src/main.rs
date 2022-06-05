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

    fn square(size: u32) -> Rectangle{
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 25,
        width: 30,
    };
    let rect2 = Rectangle {
        height: 15,
        width: 20,
    };

    let rect3 = Rectangle::square(20);
    println!("area is: {}", rect1.area());
    println!("rect2 in rect1? {}", rect1.can_hold(&rect2));
    println!("rect3 in rect2? {}", rect2.can_hold(&rect3));
    println!("rect3 in rect1? {}", rect1.can_hold(&rect3));

    _enum_destruct();
    // println!("The area is: {}", area(&rect));
}


enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn _enum_destruct(){
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The quit variant has no data to destruct", );
        }
        Message::Move{x, y} => {
            println!(
                "Move in the x direction: {} and in the y direction: {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, blue {}",
                r, g, b
            )
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation: {}, value: {}",
                h, s, v
            )
        }
    }
}
