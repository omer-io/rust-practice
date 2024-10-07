fn main() {
    let rect = Rects {
        l: 5,
        w: 5,
    };
    println!("{rect:#?}");
    let area = rect.area();
    println!("{}", area);

}

#[derive(Debug)]
struct Rects {
    l: i32,
    w: i32,
}

impl Rects {
    fn area(&self) -> i32 {
        self.l * self.w
    }
}