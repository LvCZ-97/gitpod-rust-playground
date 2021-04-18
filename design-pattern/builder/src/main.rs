fn main() {
    println!("Hello, world!");
}

pub struct Foo {
    bar: String,
}

pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new() -> Self {
        Self {
            bar: String::from('x'),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        self.bar = bar;
        self
    }
}
