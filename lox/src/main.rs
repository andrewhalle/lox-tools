use std::fmt::Display;

#[derive(Copy, Clone)]
enum Value {
    Number(f64),
}

fn num(val: f64) -> Value {
    Value::Number(val)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Value::*;

        match self {
            Number(n) => write!(f, "{}", n).unwrap(),
        }

        Ok(())
    }
}

enum Instruction {
    Push(Value),
    Print,
}

struct Vm {
    stack: Vec<Value>,
    code: Vec<Instruction>,
    ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        use Instruction::*;

        Vm {
            stack: Vec::new(),
            code: vec![Push(num(0.0)), Print],
            ip: 0,
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.code.len() {
            self.execute_one();
        }
    }

    fn execute_one(&mut self) {
        // for convenience
        use Instruction::*;

        let instruction = &self.code[self.ip];

        match instruction {
            Push(val) => {
                self.stack.push(val.clone());
            }
            Print => {
                let val = self.stack.pop().unwrap();
                println!("{}", val);
            }
        }

        self.ip += 1;
    }
}

fn main() {
    let mut vm = Vm::new();
    vm.run();
}
