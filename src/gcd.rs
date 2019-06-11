pub fn calculate_gcd(a: u64, b: u64) -> u64 {
    let mut reminder = 1;
    let mut gcd = a;
    let mut old_reminder = b;
    while reminder != 0 {
        reminder = gcd % old_reminder;
        gcd = old_reminder;
        old_reminder = reminder;
    }

    gcd
}

#[cfg(test)]
mod tests {
    use crate::gcd::calculate_gcd;

    #[test]
    fn should_calculate_gcd() {
        assert_eq!(6, calculate_gcd(78, 66));
        assert_eq!(10, calculate_gcd(10, 50));
        assert_eq!(1, calculate_gcd(17, 56));
        assert_eq!(2, calculate_gcd(170, 56));
        assert_eq!(12, calculate_gcd(1740, 456));
        assert_eq!(1, calculate_gcd(17442413, 452344236));
        assert_eq!(1, calculate_gcd(13532543663424126523, 4234637489696787542));
    }
}
