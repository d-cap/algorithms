pub struct LinearCongruentialGenerators {
    previous: u32,
    a: u32,
    b: u32,
    m: u32,
}

impl Iterator for LinearCongruentialGenerators {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.previous * self.a + self.b) % self.m;
        self.previous = next;
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::LinearCongruentialGenerators;

    #[test]
    fn should_generate_correct_sequence() {
        let mut x = LinearCongruentialGenerators {
            previous: 0,
            a: 5,
            b: 3,
            m: 7,
        };

        assert_eq!(3, x.next().unwrap());
        assert_eq!(4, x.next().unwrap());
        assert_eq!(2, x.next().unwrap());
        assert_eq!(6, x.next().unwrap());
        assert_eq!(5, x.next().unwrap());
        assert_eq!(0, x.next().unwrap());
    }
}
