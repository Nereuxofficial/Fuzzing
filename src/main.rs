use honggfuzz::fuzz;
use git_url_parse::GitUrl;

fn main() {
    println!("Starting fuzzer!");
    println!("====================");
    loop{
        fuzz!(|data: &[u8]|{
            if let Ok(s) = std::str::from_utf8(data){
                let _ = GitUrl::parse(s);
            }
        });
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn first_crash(){
        let fuzz = std::fs::read_to_string("hfuzz_workspace/Fuzzing/SIGABRT.PC.7ffff7c67d22.STACK.1bc3933b79.CODE.-6.ADDR.0.INSTR.mov____0x108(%rsp),%rax.fuzz").unwrap();
        println!("{:?}", fuzz);
        GitUrl::parse(fuzz.as_str());
    }
    #[test]
    fn second_crash(){
        let fuzz = std::fs::read_to_string("hfuzz_workspace/Fuzzing/SIGABRT.PC.7ffff7c67d22.STACK.1bdebe41c1.CODE.-6.ADDR.0.INSTR.mov____0x108(%rsp),%rax.fuzz").unwrap();
        println!("{:?}", fuzz);
        GitUrl::parse(fuzz.as_str());
    }
}