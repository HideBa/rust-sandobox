mod parent {
    pub mod child {
        pub fn print_hoge() {
            println!("hoge");
        }
    }

    mod private_child {
        pub fn print_fuga() {
            println!("fuga");
        }
    }
}

fn main() {
    use crate::parent::child::print_hoge;
    print_hoge();
}
