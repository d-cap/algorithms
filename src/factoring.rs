pub fn calculate_factoring(a: u64) -> Vec<u64> {
    let mut result = Vec::<u64>::new();
    let mut rest = a;
    while rest % 2 == 0 {
        result.push(2);
        rest /= 2;
    }

    let mut next_number = 3;

    while next_number <= rest {
        if rest % next_number == 0 {
            result.push(next_number);
            rest /= next_number;
        }

        next_number += 1;
        while !is_prime(next_number) {
            next_number += 1;
        }
    }

    if rest > 1 {
        result.push(rest);
    }

    result.sort();
    result
}

fn is_prime(n: u64) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6
    }
    true
}

#[cfg(test)]
mod tests {
    use super::calculate_factoring;
    use super::is_prime;

    #[test]
    fn should_calculate_factoring() {
        assert_eq!(vec![2, 3], calculate_factoring(6));
        assert_eq!(vec![2, 2, 3, 17], calculate_factoring(204));
        assert_eq!(
            vec![2, 3, 3, 5, 3607, 3803],
            calculate_factoring(1234567890)
        );
        //assert_eq!(vec![2, 3, 9, 21491747, 106377431], calculate_factoring(123456789012345678));
        assert_eq!(vec![5, 7, 7], calculate_factoring(245));
    }

    #[test]
    fn should_generate_correct_sequence_of_prime() {
        let mut count = 0;
        for i in 0..=10000 {
            if is_prime(i) {
                count += 1;
            }
        }
        assert_eq!(1229, count);
    }
}
