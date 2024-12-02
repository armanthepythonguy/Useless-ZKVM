use std::{collections::HashMap, fs::File, io::Write};

#[derive(Debug, Clone, PartialEq)]
pub enum Instructions{
    Push(i32),
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, Clone)]
pub struct VMState{
    stack: [i32; 4],
    instruction: Instructions
}

#[derive(Debug, Clone)]
pub struct VM{
    stack: [i32;4],
    sp: usize,
    instructions: Vec<Instructions>,
    ip: usize,
    trace: Vec<VMState>
}

impl VM{

    pub fn new(insruction: Vec<Instructions>) -> Self{
        Self { stack: [0;4], sp: 0, instructions: insruction, ip: 0, trace: vec![] }
    }

    pub fn run(&mut self) -> Result<(), String>{
        while self.ip<self.instructions.len(){
            let instruction = self.instructions[self.ip].clone();
            self.ip += 1;

            match &instruction{
                Instructions::Push(val) => {
                    for i in (1..self.stack.len()).rev(){
                        self.stack[i]=self.stack[i-1];
                    }
                    self.stack[0] = *val;
                },
                Instructions::Add => self.perform_operation(|a,b| a+b)?,
                Instructions::Sub => self.perform_operation(|a,b| a-b)?,
                Instructions::Mul => self.perform_operation(|a,b| a*b)?,
                Instructions::Div => self.perform_operation(|a,b| a/b)?,
            }

            self.trace.push(VMState { stack: self.stack, instruction: instruction });
        }

        Ok(())
    }

    fn perform_operation<F>(&mut self, operation:F) -> Result<(), String>
    where F: Fn(i32, i32)->i32
    {
        let b = self.stack[1];
        let a = self.stack[0];
        let result = operation(a,b);
        for i in 2..4{
            self.stack[i-1] = self.stack[i];
        }
        self.stack[0] = result;
        self.stack[3] = 0;
        Ok(())
    }

    pub fn get_trace(&self) -> Vec<[i32;10]>{
        let mut final_trace: Vec<[i32;10]> = vec![[0,0,0,0,0,0,0,0,0,0]];
        for i in self.trace.iter(){
            match i.instruction {
                Instructions::Push(val) => {
                    final_trace.push([i.stack[0],i.stack[1],i.stack[2],i.stack[3], val, 1, 0, 0,0,0]);
                },
                Instructions::Add =>{
                    final_trace.push([i.stack[0],i.stack[1],i.stack[2],i.stack[3], 0, 0, 1, 0,0,0]);
                },
                Instructions::Sub =>{
                    final_trace.push([i.stack[0],i.stack[1],i.stack[2],i.stack[3], 0, 0, 0, 1,0,0]);
                },
                Instructions::Mul =>{
                    final_trace.push([i.stack[0],i.stack[1],i.stack[2],i.stack[3], 0, 0, 0, 0,1,0]);
                },
                Instructions::Div =>{
                    final_trace.push([i.stack[0],i.stack[1],i.stack[2],i.stack[3], 0, 0, 0, 0,0,1]);
                }
            }
        }
        final_trace
    }

    pub fn generate_trace(&self){

        let mut final_trace: Vec<[i32;10]> = vec![[0,0,0,0,0,0,0,0,0,0]];
        for i in self.trace.iter(){
            match i.instruction {
                Instructions::Push(val) => {
                    final_trace.push([i.stack[0],i.stack[1],i.stack[2],i.stack[3], val, 1, 0, 0,0,0]);
                },
                Instructions::Add =>{
                    final_trace.push([i.stack[0],i.stack[1],i.stack[2],i.stack[3], 0, 0, 1, 0,0,0]);
                },
                Instructions::Sub =>{
                    final_trace.push([i.stack[0],i.stack[1],i.stack[2],i.stack[3], 0, 0, 0, 1,0,0]);
                },
                Instructions::Mul =>{
                    final_trace.push([i.stack[0],i.stack[1],i.stack[2],i.stack[3], 0, 0, 0, 0,1,0]);
                },
                Instructions::Div =>{
                    final_trace.push([i.stack[0],i.stack[1],i.stack[2],i.stack[3], 0, 0, 0, 0,0,1]);
                }
            }
        }
        let json = serde_json::to_string(&final_trace).unwrap();
        let mut file = File::create("trace.json").expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("JSON could not be written");

    }

}

mod tests{
    use core::error;

    use crate::vm::{Instructions, VM};


    #[test]
    fn check_add_operation(){

        let program = vec![
            Instructions::Push(10), // Push 10
            Instructions::Push(20), // Push 20
            Instructions::Add,      // Add top two values (10 + 20)
        ];

        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }

        vm.generate_trace();
        assert_eq!(vm.stack, [30,0,0,0]);
    }

    #[test]
    fn check_sub_operation(){

        let program = vec![
            Instructions::Push(10), // Push 10
            Instructions::Push(20), // Push 20
            Instructions::Sub,      // Add top two values (10 + 20)
        ];

        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }

        vm.generate_trace();
        assert_eq!(vm.stack, [10,0,0,0]);
    }

    #[test]
    fn check_mul_operation(){

        let program = vec![
            Instructions::Push(10), // Push 10
            Instructions::Push(20), // Push 20
            Instructions::Mul,      // Add top two values (10 + 20)
        ];

        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }

        vm.generate_trace();
        assert_eq!(vm.stack, [200,0,0,0]);
    }

    #[test]
    fn check_div_operation(){

        let program = vec![
            Instructions::Push(10), // Push 10
            Instructions::Push(20), // Push 20
            Instructions::Div,      // Add top two values (10 + 20)
        ];

        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }

        vm.generate_trace();
        assert_eq!(vm.stack, [2,0,0,0]);
    }

}