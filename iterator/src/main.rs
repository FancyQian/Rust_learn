struct Counter {
    counter: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next (&mut self) -> Option <Self::Item> {
        self.counter += 1;

        if self.counter < 6 {
            Some(self.counter)
        } else {
            None
        }
    }
}

impl Counter {
    fn new() -> Counter {
        Counter { counter: 0 }
    }
}

#[ test ]
fn test() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                    .map(|(a,b)| a * b)
                    .filter(|x| x % 3 ==0)
                    .sum();
    assert_eq!(sum, 18)
}
