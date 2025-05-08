mod p3;
mod vm;

mod test {
    
    

    

    #[test]
    fn test_end_to_end() {
        //Generating the trace for the required program
        let program = vec![
            Instructions::Push(Mersenne31::from_canonical_u32(10)), // Push 10
            Instructions::Push(Mersenne31::from_canonical_u32(20)), // Push 20
            Instructions::Add,
            Instructions::Push(Mersenne31::from_canonical_u32(40)), // Push 40
            Instructions::Sub,
            Instructions::Push(Mersenne31::from_canonical_u32(2)), // Push 2
            Instructions::Mul,
            Instructions::Push(Mersenne31::from_canonical_u32(23)), // Push 23
            Instructions::Div,
        ];
        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }

        //Generating proofs for the program
        let vmair = VMAir {};
        vmair.generate_proof(vm);
    }
}
