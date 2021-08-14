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
        let fuzz = std::fs::read_to_string("hfuzz_workspace/Fuzzing/SIGABRT.PC.7ffff7c67d22.STACK.1a41fbdb78.CODE.-6.ADDR.0.INSTR.mov____0x108(%rsp),%rax.fuzz").unwrap();
        println!("{:?}", fuzz);
        eval_str(fuzz.as_str());
    }

    #[test]
    fn second_crash() {
        let fuzz = std::fs::read_to_string("hfuzz_workspace/Fuzzing/SIGABRT.PC.7ffff7c67d22.STACK.f566d96db.CODE.-6.ADDR.0.INSTR.mov____0x108(%rsp),%rax.fuzz").unwrap();
        println!("{:?}", fuzz);
        eval_str(fuzz.as_str());
    }
    #[test]
    fn third_crash() {
        let fuzz = std::fs::read_to_string("hfuzz_workspace/Fuzzing/SIGABRT.PC.7ffff7c67d22.STACK.f482151fd.CODE.-6.ADDR.0.INSTR.mov____0x108(%rsp),%rax.fuzz").unwrap();
        println!("{:?}", fuzz);
        eval_str(fuzz.as_str());
    }
}
