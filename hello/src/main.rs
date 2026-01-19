struct A {
    value: String,
}

impl A {
    pub fn new(value: &str) -> A {
        Self {
            value: value.into(),
        }
    }
}

fn main() {
    let x = String::from("adam");
    let a = A::new(&x);
}
