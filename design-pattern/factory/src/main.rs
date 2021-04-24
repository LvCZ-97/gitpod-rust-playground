enum ProductType {
    Product1,
    Product2,
}

trait Product {
    fn show(&self);
}

trait Factory {
    fn make_product(&self, product_type: ProductType) -> Box<dyn Product>;
}

struct ConcreteProduct1(String);
impl Product for ConcreteProduct1 {
    fn show(&self) {
        println!("red color, {}", self.0);
    }
}

struct ConcreteProduct2(String);
impl Product for ConcreteProduct2 {
    fn show(&self) {
        println!("blue color, {}", self.0);
    }
}

struct SimpleFactory;
impl SimpleFactory {
    fn new() -> Self {
        Self
    }
}
impl Factory for SimpleFactory {
    fn make_product(&self, color_type: ProductType) -> Box<dyn Product> {
        match color_type {
            ProductType::Product1 => Box::new(ConcreteProduct1("red".to_string())),
            ProductType::Product2 => Box::new(ConcreteProduct2("blue".to_string())),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let factory = SimpleFactory::new();
    let product = factory.make_product(ProductType::Product1);
    product.show();
    let product = factory.make_product(ProductType::Product2);
    product.show();
}
