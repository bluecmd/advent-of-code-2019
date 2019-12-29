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
            let mut found_two = false;
            for (p, c) in s.chars().zip(s.chars().skip(1)) {
                if p == c {
                    found_two = true;
                    break;
                }
            }
            if !found_two {
                continue;
            }
            return Some(self.current);
        }
    }
}

fn main() {
    let pi = PasswordIterator {
        current: 234208,
        stop: 765869,
    };
    println!("Number of passwords={}", pi.count());
}
