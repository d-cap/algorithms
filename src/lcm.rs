use crate::gcd::calculate_gcd;

pub fn calculate_lcm(a: u64, b: u64) -> u64 {
    a * (b / calculate_gcd(a, b))
}

#[cfg(test)]
mod tests {
    use crate::lcm::calculate_lcm;

    #[test]
    fn should_calculate_lcm() {
        assert_eq!(60, calculate_lcm(12, 15));
        assert_eq!(1716, calculate_lcm(132, 156));
        assert_eq!(20000, calculate_lcm(10000, 20000));
        assert_eq!(10005276862774952840, calculate_lcm(10003214140, 20004124120));
    }
}