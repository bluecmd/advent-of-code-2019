use std::io;

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Exit(),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Program {
    data: Vec<usize>,
    ip: usize,
}

impl Program {
    fn from(data: &[usize]) -> Program {
        Program {
            data: data.to_vec(),
            ip: 0,
        }
    }

    fn parse(text: &str) -> Program {
        let data: Vec<usize> = text
            .split(',')
            .map(|x| {
                x.trim()
                    .parse()
                    .expect(&format!("Number parse failed: '{}'", x))
            })
            .collect();
        Program::from(&data)
    }

    fn advance(&mut self) -> usize {
        self.ip += 1;
        let d = self.data.get(self.ip - 1);
        match d {
            Some(d) => *d,
            None => panic!("Execution @ IP={} failed, EOF", self.ip - 1),
        }
    }

    fn execute(&mut self) -> &[usize] {
        // NOTE: It appears we cannot use a for-loop here since it will
        // demand ownership of the iterator (self in our case) for the duration
        // of the loop, making it impossible for us to support self-modifying
        // code.
        loop {
            match self.next() {
                Some(op) => match op {
                    Opcode::Add(a, b, res) => self.data[res] = self.data[a] + self.data[b],
                    Opcode::Multiply(a, b, res) => self.data[res] = self.data[a] * self.data[b],
                    Opcode::Exit() => break,
                },
                None => break,
            }
        }
        &self.data[..]
    }
}

impl Iterator for Program {
    type Item = Opcode;
    fn next(&mut self) -> Option<Opcode> {
        match self.advance() {
            1 => Some(Opcode::Add(self.advance(), self.advance(), self.advance())),
            2 => Some(Opcode::Multiply(
                self.advance(),
                self.advance(),
                self.advance(),
            )),
            99 => Some(Opcode::Exit()),
            _ => panic!("Encountered invalid opcode @ IP={}", self.ip - 1),
        }
    }
}

fn main() {
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read stdin");
    let golden = Program::parse(&text);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = golden.clone();
            program.data[1] = noun;
            program.data[2] = verb;
            if program.execute()[0] == 19690720 {
                println!("Result={}", noun * 100 + verb);
                break;
            }
        }
    }
}

#[test]
fn parse_works() {
    assert_eq!(
        Program::parse("1,2,3"),
        Program {
            data: [1, 2, 3].to_vec(),
            ip: 0,
        }
    );
}

#[test]
fn opcode_works() {
    assert_eq!(Program::parse("1,2,3,4").next(), Some(Opcode::Add(2, 3, 4)));
}

#[test]
fn execute_works() {
    let mut p = Program::from(&[1, 0, 0, 0, 99]);
    let result = p.execute();
    assert_eq!(result, [2, 0, 0, 0, 99]);
}

#[test]
fn reference_check() {
    assert_eq!(Program::parse("2,3,0,3,99").execute(), [2, 3, 0, 6, 99]);
    assert_eq!(
        Program::parse("2,4,4,5,99,0").execute(),
        [2, 4, 4, 5, 99, 9801]
    );
    assert_eq!(
        Program::parse("1,1,1,4,99,5,6,0,99").execute(),
        [30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
}
