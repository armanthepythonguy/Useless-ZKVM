use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{extension::BinomialExtensionField, Field};
use p3_keccak::Keccak256Hash;
use p3_matrix::Matrix;
use p3_mersenne_31::Mersenne31;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher32};
use p3_uni_stark::Proof;

pub struct VMAir{}

impl<F: Field> BaseAir<F> for VMAir{

    fn width(&self) -> usize {
        10
    }

}

impl<AB: AirBuilder> Air<AB> for VMAir{

    fn eval(&self, builder: &mut AB) {
        
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);

        builder.when_first_row().assert_zero(local[0]+local[1]+local[2]+local[3]+local[4]+local[5]+local[6]+local[7]+local[8]+local[9]);
        
        //Constraints for add computation
        builder.when_transition().assert_zero(next[6]*(next[0]-local[0]-local[1]));
        builder.when_transition().assert_zero(next[6]*(next[1]-local[2]));

        //Constraints for sub computation
        builder.when_transition().assert_zero(next[7]*(local[0]-local[1]-next[0]));

        //Constraint for mul computation
        builder.when_transition().assert_zero(next[8]*(local[0]*local[1]-next[0]));

        // //Constraint for div computation
        // builder.when_transition().assert_zero(next[9]*(local[0]/local[1]-next[0]));
        
        //Constraints for push computation
        builder.when_transition().assert_zero(next[5]*(next[0]-next[4])*(local[0]-next[1])*(local[1]-next[2])*(local[2]-next[3]));

        builder.when_transition().assert_zero((next[6]+next[7]+next[8]+next[9])*(next[1]-local[2]));
    }

}

impl VMAir{

    pub fn generate_proof(){

        type Val = Mersenne31;
        type Challenge = BinomialExtensionField<Val,3>;
        type ByteHash = Keccak256Hash;
        type FieldHash = SerializingHasher32<ByteHash>;
        let byte_hash = ByteHash{};
        let field_hash = FieldHash::new(Keccak256Hash{});

        type MyCompress = CompressionFunctionFromHasher<ByteHash, 2, 32>;
        type ValMmcs = FieldMer

    }

}