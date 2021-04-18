#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}

pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new() -> Self {
        Self {
            bar: String::from("x"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        self.bar = bar;
        self
    }

    pub fn build(self) -> Foo {
        Foo {
            bar: self.bar
        }
    }
}

fn main() {
    println!("Hello, world!");

    let foo = Foo {
        bar: String::from("Y"),
    };
    let foo_from_builder = FooBuilder::new().name(String::from("Y")).build();
    println!("foo = {:?}", foo);
    println!("foo from builder = {:?}", foo_from_builder);
}
