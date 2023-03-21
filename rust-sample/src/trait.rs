struct User {
    name: String,
    age: u32,
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.age)
    }
}

trait Area {
    fn area(&self) -> u32;
}

struct Square {
    width: u32,
    height: u32,
}

impl Area for Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Square {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let square = Square::new(10, 20);

    println!("{}", square.area());
}

// トレイト境界
fn compare_are(a: impl Area, b: impl Area) -> bool {
    a.area() == b.area()
}

// 以下は糖衣構文
fn compare_are2<T: Area>(a: T, b: T) -> bool {
    a.area() == b.area()
}
