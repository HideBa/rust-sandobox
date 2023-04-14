fn main() {
    let some: Result<&str, &str> = Ok("Hello, world!");
    println!("{:?}", some);

    let err: Result<&str, &str> = Err("Error");
    println!("{:?}", err);
}

fn always_error() -> Result<(), String> {
    Err("always error".to_string())
}

fn might_fail() -> Result<(), String> {
    let _result = always_error()?;
}

fn hello_print(message: &str) {
    println!("{}", message);
}

fn main2() {
    let word = String::from("Hello, world!");
    hello_print(&word);
    println!("{}", word);
}

fn box_test() {
    let a = Box::new("test");
    println!("{}", a);
}
