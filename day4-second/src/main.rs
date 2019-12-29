#![feature(is_sorted)]

struct PasswordIterator {
    current: u32,
    stop: u32,
}

impl Iterator for PasswordIterator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        loop {
            if self.current == self.stop {
                return None;
            }
            self.current += 1;
            let s = self.current.to_string();
            if !s.chars().is_sorted() {
                continue;
            }
            if !is_valid(&s) {
                continue;
            }
            return Some(self.current);
        }
    }
}

fn is_valid(s: &str) -> bool {
    let mut last_index = 0;
    for (i, (p, c)) in s.chars().zip(s.chars().skip(1)).enumerate() {
        if p != c {
            if i - last_index == 1 {
                return true;
            }
            last_index = i + 1;
        }
    }
    s.len() - last_index == 2
}

#[test]
fn test_reference() {
    assert_eq!(is_valid("112233"), true);
    assert_eq!(is_valid("123444"), false);
    assert_eq!(is_valid("111122"), true);
}

fn main() {
    let pi = PasswordIterator {
        current: 234208,
        stop: 765869,
    };
    println!("Number of passwords={}", pi.count());
}
