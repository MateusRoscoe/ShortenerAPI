pub fn to_base62(num: u64) -> String {
    let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let base = chars.len() as u64;
    let mut num = num + 238328; // 238328 guarantees at least 4 digits
    let mut result = String::new();

    while num > 0 {
        let remainder = (num % base) as usize;
        num /= base;
        result.push(chars.chars().nth(remainder).unwrap());
    }

    result.chars().rev().collect::<String>()
}
