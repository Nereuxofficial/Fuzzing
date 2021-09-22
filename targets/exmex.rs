use exmex::eval_str;
use honggfuzz::fuzz;

fn main() {
    println!("Starting fuzzer!");
    println!("====================");
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let _: f32 = eval_str(s).unwrap();
            }
        });
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;
}