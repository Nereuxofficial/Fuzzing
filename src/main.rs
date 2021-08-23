use exmex::eval_str;
use honggfuzz::fuzz;

fn main() {
    println!("Starting fuzzer!");
    println!("====================");
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let _ = eval_str(s);
            }
        });
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;

    #[test]
    fn first_crash() {
        // Probably irrelevant
        // Expected binary operator but there was none
        let fuzz = std::fs::read_to_string("hfuzz_workspace/Fuzzing/SIGABRT.PC.7ffff7c67d22.STACK.1a404299a5.CODE.-6.ADDR.0.INSTR.mov____0x108(%rsp),%rax.fuzz").unwrap();
        println!("{:?}", fuzz);
        eval_str(fuzz.as_str());
    }

    #[test]
    fn second_crash() {
        // Probably also irrelevant(ParseFloatError)
        let fuzz = std::fs::read_to_string("hfuzz_workspace/Fuzzing/SIGABRT.PC.7ffff7c67d22.STACK.e4a70c637.CODE.-6.ADDR.0.INSTR.mov____0x108(%rsp),%rax.fuzz").unwrap();
        println!("{:?}", fuzz);
        eval_str(fuzz.as_str());
    }
    #[test]
    fn third_crash() {
        // attempt to add with overflow
        let fuzz = std::fs::read_to_string("hfuzz_workspace/Fuzzing/SIGABRT.PC.7ffff7c67d22.STACK.e543c0111.CODE.-6.ADDR.0.INSTR.mov____0x108(%rsp),%rax.fuzz").unwrap();
        println!("{:?}", fuzz);
        eval_str(fuzz.as_str());
    }
    #[test]
    fn fourth_crash(){
        // Also crashes at num_str.parse(ParseFloatError)
        let fuzz = std::fs::read_to_string("hfuzz_workspace/Fuzzing/SIGABRT.PC.7ffff7c67d22.STACK.e4a70c637.CODE.-6.ADDR.0.INSTR.[NOT_MMAPED].fuzz").unwrap();
        println!("{:?}", fuzz);
        eval_str(fuzz.as_str());
    }
    #[test]
    fn fifth_crash(){
        // byte index 7 is not a char boundary
        // It is assumed that index 7 is a char, which in this case is not given
        let fuzz = std::fs::read_to_string("hfuzz_workspace/Fuzzing/SIGABRT.PC.7ffff7c67d22.STACK.ec628af68.CODE.-6.ADDR.0.INSTR.[NOT_MMAPED].fuzz").unwrap();
        println!("{:?}", fuzz);
        eval_str(fuzz.as_str());
    }
}
