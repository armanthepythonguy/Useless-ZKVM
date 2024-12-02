mod p3;
mod vm;

mod test {
    use crate::{
        p3::VMAir,
        vm::{Instructions, VM},
    };

    #[test]
    fn test_end_to_end() {
        //Generating the trace for the required program
        let program = vec![
            Instructions::Push(10),
            Instructions::Push(20),
            Instructions::Add,
            Instructions::Push(40),
            Instructions::Sub,
            Instructions::Push(2),
            Instructions::Mul,
            Instructions::Push(23),
            Instructions::Div,
        ];
        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }
        vm.generate_trace();

        //Generating proofs for the program
        let vmair = VMAir {};
        vmair.generate_proof(vm);
    }
}
