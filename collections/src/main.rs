mod math;
mod departments;

fn main() {
    let vector = vec![5,6,7,10, 4,6 ,5];
    println!("Average: {}", math::average(&vector));
    println!("Median: {}", math::median(&vector));
    println!("Mode: {}", math::mode(&vector));

    for (index, value) in vector.iter().enumerate() {
        println!("Element {}: {}", index, value);
    }

    let mut manager = departments::DepartmentsManager::new();
    manager.accept_input();
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main1() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

