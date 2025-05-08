
use p3_field::FieldAlgebra;
use p3_mersenne_31::Mersenne31;

#[derive(Debug, Clone, PartialEq)]
pub enum Instructions {
    Push(Mersenne31),
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub struct VMState {
    stack: [Mersenne31; 4],
    instruction: Instructions,
    extra_data: Mersenne31,
}

#[derive(Debug, Clone)]
pub struct VM {
    stack: [Mersenne31; 4],
    sp: usize,
    instructions: Vec<Instructions>,
    ip: usize,
    trace: Vec<VMState>,
}

impl VM {
    pub fn new(insruction: Vec<Instructions>) -> Self {
        Self {
            stack: [Mersenne31::ZERO; 4],
            sp: 0,
            instructions: insruction,
            ip: 0,
            trace: vec![],
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.ip < self.instructions.len() {
            let instruction = self.instructions[self.ip].clone();
            self.ip += 1;

            type BinaryOp = fn(Mersenne31, Mersenne31) -> Mersenne31;
            let mut extra_data: Mersenne31 = Mersenne31::ZERO;
            match &instruction {
                Instructions::Push(val) => {
                    for i in (1..self.stack.len()).rev() {
                        self.stack[i] = self.stack[i - 1];
                    }
                    self.stack[0] = *val;
                }
                Instructions::Add => {
                    self.perform_operation(|a, b| a + b, None::<BinaryOp>)?;
                }
                Instructions::Sub => {
                    self.perform_operation(|a, b| a - b, None::<BinaryOp>)?;
                }
                Instructions::Mul => {
                    self.perform_operation(|a, b| a * b, None::<BinaryOp>)?;
                }
                Instructions::Div => {
                    extra_data =
                        self.perform_operation(|a, b| a / b, Some(|a, b| a - (a / b) * b))?;
                }
            }

            self.trace.push(VMState {
                stack: self.stack,
                instruction: instruction,
                extra_data: extra_data,
            });
        }

        Ok(())
    }

    fn perform_operation<F, G>(
        &mut self,
        operation: F,
        sub_operation: Option<G>,
    ) -> Result<Mersenne31, String>
    where
        F: Fn(Mersenne31, Mersenne31) -> Mersenne31,
        G: Fn(Mersenne31, Mersenne31) -> Mersenne31,
    {
        let b = self.stack[1];
        let a = self.stack[0];
        let result = operation(a, b);
        let mut extra_data: Mersenne31 = Mersenne31::ZERO;
        match sub_operation {
            Some(f) => {
                extra_data = f(a, b);
            }
            None => {}
        }
        for i in 2..4 {
            self.stack[i - 1] = self.stack[i];
        }
        self.stack[0] = result;
        self.stack[3] = Mersenne31::ZERO;
        Ok(extra_data)
    }

    pub fn get_trace(&self) -> Vec<[Mersenne31; 11]> {
        let mut final_trace: Vec<[Mersenne31; 11]> = vec![[Mersenne31::ZERO; 11]];
        for i in self.trace.iter() {
            match i.instruction {
                Instructions::Push(val) => {
                    final_trace.push([
                        i.stack[0],
                        i.stack[1],
                        i.stack[2],
                        i.stack[3],
                        val,
                        Mersenne31::ONE,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        i.extra_data,
                    ]);
                }
                Instructions::Add => {
                    final_trace.push([
                        i.stack[0],
                        i.stack[1],
                        i.stack[2],
                        i.stack[3],
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ONE,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        i.extra_data,
                    ]);
                }
                Instructions::Sub => {
                    final_trace.push([
                        i.stack[0],
                        i.stack[1],
                        i.stack[2],
                        i.stack[3],
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ONE,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        i.extra_data,
                    ]);
                }
                Instructions::Mul => {
                    final_trace.push([
                        i.stack[0],
                        i.stack[1],
                        i.stack[2],
                        i.stack[3],
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ONE,
                        Mersenne31::ZERO,
                        i.extra_data,
                    ]);
                }
                Instructions::Div => {
                    final_trace.push([
                        i.stack[0],
                        i.stack[1],
                        i.stack[2],
                        i.stack[3],
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ONE,
                        i.extra_data,
                    ]);
                }
            }
        }
        final_trace
    }
}

mod tests {
    

    
    

    

    #[test]
    fn check_add_operation() {
        let program = vec![
            Instructions::Push(Mersenne31::from_canonical_u32(10)), // Push 10
            Instructions::Push(Mersenne31::from_canonical_u32(20)), // Push 20
            Instructions::Add,                                      // Add top two values (10 + 20)
        ];

        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }

        assert_eq!(
            vm.stack,
            [
                Mersenne31::from_canonical_u32(30),
                Mersenne31::ZERO,
                Mersenne31::ZERO,
                Mersenne31::ZERO
            ]
        );
    }

    #[test]
    fn check_sub_operation() {
        let program = vec![
            Instructions::Push(Mersenne31::from_canonical_u32(10)), // Push 10
            Instructions::Push(Mersenne31::from_canonical_u32(20)), // Push 20
            Instructions::Sub,                                      // Sub top two values (20-10)
        ];

        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }

        assert_eq!(
            vm.stack,
            [
                Mersenne31::from_canonical_u32(10),
                Mersenne31::ZERO,
                Mersenne31::ZERO,
                Mersenne31::ZERO
            ]
        );
    }

    #[test]
    fn check_mul_operation() {
        let program = vec![
            Instructions::Push(Mersenne31::from_canonical_u32(10)), // Push 10
            Instructions::Push(Mersenne31::from_canonical_u32(20)), // Push 20
            Instructions::Mul,                                      // Mul top two values (10 * 20)
        ];

        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }

        assert_eq!(
            vm.stack,
            [
                Mersenne31::from_canonical_u32(200),
                Mersenne31::ZERO,
                Mersenne31::ZERO,
                Mersenne31::ZERO
            ]
        );
    }

    #[test]
    fn check_div_operation() {
        let program = vec![
            Instructions::Push(Mersenne31::from_canonical_u32(10)), // Push 10
            Instructions::Push(Mersenne31::from_canonical_u32(20)), // Push 20
            Instructions::Div,                                      // Div top two values (20/10)
        ];

        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }

        assert_eq!(
            vm.stack,
            [
                Mersenne31::from_canonical_u32(2),
                Mersenne31::ZERO,
                Mersenne31::ZERO,
                Mersenne31::ZERO
            ]
        );
    }
}
