use std::string;

pub struct Test {
    pub name: String,
    pub num: i32,
}

impl Test {
    pub fn new() -> Test {
        Test {
            name: "n1".to_owned(),
            num: 1,
        }
    }
    pub fn newaa() {
        println!("it is ok.")
    }
    pub fn newbb(&self) {
        println!("it is bb.{}", self.name)
    }
}

fn main() {
    println!("hi,~je!");
    let t = Test::new();
    t.newbb();

    let t2 = Test {
        name: "n2".to_owned(),
        num: 2,
    };

    t2.newbb();
}
