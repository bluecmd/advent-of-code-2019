use std::io;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Vector {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

type VectorList = Vec<Vector>;

trait VectorOps {
    fn bounding_box(&self) -> (usize, usize);
}

impl VectorOps for VectorList {
    fn bounding_box(&self) -> (usize, usize) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for v in self {
            match v {
                Vector::Up(l) => y -= *l as i32,
                Vector::Down(l) => y += *l as i32,
                Vector::Right(l) => x += *l as i32,
                Vector::Left(l) => x -= *l as i32,
            }
            if y > height {
                height = y
            }
            if 0 - y > height {
                height = 0 - y
            }
            if x > width {
                width = x
            }
            if 0 - x > width {
                width = 0 - x
            }
        }
        // Return the absolute maximum width and height to keep the starting
        // point in the middle and still fit all operations
        (width as usize, height as usize)
    }
}

fn parse_vector_list(text: &str) -> VectorList {
    text.trim().split(',').map(Vector::parse).collect()
}

impl Vector {
    fn parse(text: &str) -> Vector {
        let mut chars = text.chars();
        let dir = chars.next().expect("Invalid direction format");
        let len = chars.as_str();
        let len = len.parse::<usize>().expect("Vector length not numeric");
        match dir {
            'U' => Vector::Up(len),
            'D' => Vector::Down(len),
            'L' => Vector::Left(len),
            'R' => Vector::Right(len),
            _ => panic!("Invalid direction: {}", dir),
        }
    }

    fn len(&self) -> usize {
        match self {
            Vector::Up(l) => *l,
            Vector::Down(l) => *l,
            Vector::Right(l) => *l,
            Vector::Left(l) => *l,
        }
    }

    fn decrement(&mut self) {
        match self {
            Vector::Up(ref mut l) => *l -= 1,
            Vector::Down(ref mut l) => *l -= 1,
            Vector::Right(ref mut l) => *l -= 1,
            Vector::Left(ref mut l) => *l -= 1,
        }
    }

    fn zero() -> Vector {
        Vector::Up(0)
    }
}

struct Pallet {
    data: Vec<i8>,
    width: usize,
    height: usize,
    mask: i8,
}

impl Pallet {
    fn new(width: usize, height: usize) -> Pallet {
        Pallet {
            data: vec![0; width * 2 * height * 2],
            mask: 1,
            width,
            height,
        }
    }

    fn mark(&mut self, x: i32, y: i32) {
        let offx = (self.width as i32 + x) as usize;
        let offy = (self.height as i32 + y) as usize;
        self.data[offx + offy * self.width * 2] |= self.mask;
    }

    fn draw(&mut self, vl: &VectorList) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut i = vl.iter();
        let mut v: Vector = Vector::zero();
        loop {
            if v.len() == 0 {
                let ov = i.next();
                v = if ov == None {
                    break;
                } else {
                    ov.unwrap().clone()
                };
            }
            // Single-step painting
            self.mark(x, y);
            match v {
                Vector::Up(_) => y -= 1,
                Vector::Down(_) => y += 1,
                Vector::Right(_) => x += 1,
                Vector::Left(_) => x -= 1,
            }
            v.decrement();
        }
        self.mask *= 2;
    }

    fn print(&mut self) {
        let md = self.min_crosspoint_distance();
        print!("|");
        for _ in 0..self.width * 2 {
            print!("-");
        }
        println!("|");
        for (i, v) in self.data.iter().enumerate() {
            if i % (self.width * 2) == 0 {
                print!("|");
            }
            if i == (self.width * 2 * self.height) + self.width {
                print!("o");
            } else {
                let offx = i % (self.width * 2);
                let offy = i / (self.width * 2);
                let distance = ((offx as i32 - self.width as i32).abs()
                    + (offy as i32 - self.height as i32).abs())
                    as usize;
                let ones = v.count_ones();
                if md != None && ones > 1 && md.unwrap() == distance {
                    print!("*");
                } else {
                    match ones {
                        0 => print!(" "),
                        1 => print!("x"),
                        2 => print!("!"),
                        _ => print!("?"),
                    }
                }
            }
            if i % (self.width * 2) == self.width * 2 - 1 {
                println!("|");
            }
        }
        print!("|");
        for _ in 0..self.width * 2 {
            print!("-");
        }
        println!("|");
    }

    fn min_crosspoint_distance(&mut self) -> Option<usize> {
        let mut ds = Vec::new();
        for (i, _) in self
            .data
            .iter()
            .enumerate()
            .filter(|(_, b)| b.count_ones() > 1)
        {
            // Origo is not counted
            if i == (self.width * 2 * self.height) + self.width {
                continue;
            }
            let offx = i % (self.width * 2);
            let offy = i / (self.width * 2);
            let distance = ((offx as i32 - self.width as i32).abs()
                + (offy as i32 - self.height as i32).abs()) as usize;
            ds.push(distance);
        }
        match ds.iter().min() {
            Some(v) => Some(*v),
            None => None,
        }
    }
}

fn solve(s1: &str, s2: &str) -> Option<usize> {
    let v1 = parse_vector_list(s1);
    let v2 = parse_vector_list(s2);
    let (w1, h1) = v1.bounding_box();
    let (w2, h2) = v2.bounding_box();
    let width = if w1 > w2 { w1 } else { w2 };
    let height = if h1 > h2 { h1 } else { h2 };
    let mut pallet = Pallet::new(width, height);
    pallet.draw(&v1);
    pallet.draw(&v2);
    pallet.min_crosspoint_distance()
}

fn main() {
    let mut first = String::new();
    let mut second = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("Failed to read first line");
    io::stdin()
        .read_line(&mut second)
        .expect("Failed to read second line");
    let d = solve(&first, &second);
    println!("Closest crosspoint={:?}", d);
}

#[test]
fn parse_vector() {
    assert_eq!(Vector::parse("R100"), Vector::Right(100))
}

#[test]
fn bounding_box() {
    let v: Vec<Vector> = [Vector::parse("R100"), Vector::parse("U50")].to_vec();
    assert_eq!(v.bounding_box(), (100, 50))
}

#[test]
fn reference_1_test() {
    assert_eq!(solve("U7,R6,D4,L4", "R8,U5,L5,D3"), Some(6));
}

#[test]
fn reference_2_test() {
    assert_eq!(
        solve(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83"
        ),
        Some(159)
    );
}

#[test]
fn reference_3_test() {
    assert_eq!(
        solve(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        ),
        Some(135)
    );
}
