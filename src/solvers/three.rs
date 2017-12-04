use std::cmp;

#[allow(dead_code)]
pub const INPUT: i32 = 347991;

#[allow(dead_code)]
pub fn a(input: i32) -> i32 {
    let sqrt = f64::from(input).sqrt() as i32;
    let nearest_sq = sqrt.pow(2);
    if nearest_sq == input {
        sqrt - 1
    } else {
        let focal_dist = (sqrt + 1) / 2;
        let focal_a = nearest_sq + sqrt / 2 + 1;
        let focal_b = nearest_sq + sqrt + focal_dist + 1;
        focal_dist + cmp::min((input - focal_a).abs(), (focal_b - input).abs())
    }
}

#[allow(dead_code)]
pub fn b(input: u32) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(1), 0);
        assert_eq!(a(12), 3);
        assert_eq!(a(23), 2);
        assert_eq!(a(1024), 31);
    }
}
