struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn rename(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    let mut user = User::new(String::from("John"), 20);
    user.rename(String::from("Mike"));
    println!("{} {}", user.name, user.age);
}
