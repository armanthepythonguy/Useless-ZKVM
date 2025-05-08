mod p3;
mod vm;

#[cfg(test)]
mod test {
    use p3_field::PrimeCharacteristicRing;
    use p3_mersenne_31::Mersenne31;

    use crate::{
        p3::VMAir,
        vm::{Instructions, VM},
    };
    #[test]
    fn test_end_to_end() {
        //Generating the trace for the required program
        let program = vec![
            Instructions::Push(Mersenne31::from_u32(10)), // Push 10
            Instructions::Push(Mersenne31::from_u32(20)), // Push 20
            Instructions::Add,
            Instructions::Push(Mersenne31::from_u32(40)), // Push 40
            Instructions::Sub,
            Instructions::Push(Mersenne31::from_u32(2)), // Push 2
            Instructions::Mul,
            Instructions::Push(Mersenne31::from_u32(23)), // Push 23
            Instructions::Div,
        ];
        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("{}", error);
            return;
        }

        //Generating proofs for the program
        let vmair = VMAir {};
        vmair.generate_proof(vm).expect("end-to-end proof generation")
    }
}
