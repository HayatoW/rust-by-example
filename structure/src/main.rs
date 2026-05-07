fn main() {
    println!("Hello, world!");
}

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = self;

        (x1 - x2).abs() * (y1 - y2).abs()
    }
}

#[test]
fn test_rect_area() {
    let rectangle = Rectangle {
        top_left: Point { x: 0f32, y: 2f32 },
        bottom_right: Point { x: 2f32, y: 0f32 },
    };

    assert_eq!(rectangle.rect_area(), 4f32);
}
