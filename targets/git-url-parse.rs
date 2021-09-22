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
}