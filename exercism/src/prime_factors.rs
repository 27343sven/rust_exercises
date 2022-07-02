#[allow(dead_code)]
#[allow(unused)]

struct PrimeGenerator {
    cache: Vec<u64>,
    index: Option<usize>,
}

impl Iterator for PrimeGenerator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.index {
            Some(i) if i + 1 < self.cache.len() => *self.cache.get(i + 1).unwrap(),
            None if !self.cache.is_empty() => *self.cache.first().unwrap(),
            _ => {
                let next = self.generate_next();
                self.cache.push(next);
                next
            }
        };
        self.index = match self.index {
            Some(i) => Some(i + 1),
            None => Some(0),
        };
        Some(next)
    }
}

impl PrimeGenerator {
    fn new() -> Self {
        PrimeGenerator {
            cache: Vec::new(),
            index: None,
        }
    }

    fn reset(&mut self) {
        self.index = None;
    }

    fn generate_next(&mut self) -> u64 {
        if self.cache.is_empty() {
            self.index = Some(0);
            2
        } else {
            let mut first: u64 = *self.cache.last().unwrap();
            match first {
                2 => 3,
                3 => 5,
                _ => loop {
                    first += match first % 6 {
                        1 => 4,
                        _ => 2,
                    };
                    if is_prime(first) {
                        break first;
                    }
                },
            }
        }
    }
}

pub fn is_prime(n: u64) -> bool {
    if let 2 | 3 = n {
        true
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        !(5..=(n as f64).sqrt().ceil() as u64)
            .step_by(6)
            .any(|x| n % x == 0 || n % (x + 2) == 0)
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut generator = PrimeGenerator::new();
    let mut result = Vec::new();
    let mut curr = n;
    while curr > 1 {
        while let Some(p) = generator.next() {
            if curr % p == 0 {
                curr = curr / p;
                result.push(p);
                break;
            } else if p > curr {
                break;
            }
        }
        generator.reset();
    }
    result
}

mod tests {
    use super::*;

    #[test]
    fn factors_of_60() {
        let mut generator = PrimeGenerator::new();
        let primes = (0..9).map(|_| generator.next()).collect::<Vec<_>>();
        println!("{:?}", primes);
        generator.reset();
        let primes = (0..6).map(|_| generator.next()).collect::<Vec<_>>();
        println!("{:?}", primes);
        // println!("{:?}", factors(60));
    }

    #[test]
    fn test_no_factors() {
        assert_eq!(factors(1), vec![]);
    }

    #[test]
    fn test_prime_number() {
        assert_eq!(factors(2), vec![2]);
    }

    #[test]
    fn test_square_of_a_prime() {
        assert_eq!(factors(9), vec![3, 3]);
    }

    #[test]
    fn test_cube_of_a_prime() {
        assert_eq!(factors(8), vec![2, 2, 2]);
    }

    #[test]
    fn test_product_of_primes_and_non_primes() {
        assert_eq!(factors(12), vec![2, 2, 3]);
    }

    #[test]
    fn test_product_of_primes() {
        assert_eq!(factors(901_255), vec![5, 17, 23, 461]);
    }

    #[test]
    fn test_factors_include_large_prime() {
        assert_eq!(factors(93_819_012_551), vec![11, 9539, 894_119]);
    }
}
