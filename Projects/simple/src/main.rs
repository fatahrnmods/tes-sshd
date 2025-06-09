trait Describable {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {}", self.title)
    }
}

struct Magazine {
    name: String,
}

impl Describable for Magazine {
    fn describe(&self) -> String {
        format!("Magazine: {}", self.name)
    }
}

fn main() {
    let items: Vec<Box<dyn Describable>> = vec![
        Box::new(Book { title: String::from("Rust Book") }),
        Box::new(Magazine { name: String::from("Rust Monthly") }),
    ];

    for item in items {
        println!("{}", item.describe());
    }
}
