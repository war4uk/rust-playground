fn main() {
    let sum: u32 = Counter::new().take(5)
                                .zip(Counter::new().skip(1))
                                .map(|(a, b)| a * b)
                                .filter(|x| x % 3 == 0)
                                .sum();
    assert_eq!(18, sum);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count > 5 {
            None
        } else {
            Some(self.count)
        }
    }
}