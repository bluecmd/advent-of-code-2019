use std::io;

struct Input {
    buf: String,
}

impl Input {
    fn new() -> Input {
        Input { buf: String::new() }
    }
}

impl Iterator for Input {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();
        io::stdin()
            .read_line(&mut self.buf)
            .expect("Stdin read failed");
        if self.buf.len() == 0 {
            None
        } else {
            Some(self.buf.trim().parse().expect("Invalid number format"))
        }
    }
}

fn main() {
    let input = Input::new();
    let answer: u32 = input.map(fuel_required).sum();
    println!("Fuel required: {}", answer);
}

fn fuel_required(mass: u32) -> u32 {
    let fuel = (mass as i32 / 3) - 2;
    let done = fuel <= 0;
    let fuel = fuel as u32;
    if !done {
        fuel + fuel_required(fuel)
    } else {
        0
    }
}

#[test]
fn it_works() {
    assert_eq!(fuel_required(12), 2);
    assert_eq!(fuel_required(14), 2);
    assert_eq!(fuel_required(1969), 966);
    assert_eq!(fuel_required(100756), 50346);
}
