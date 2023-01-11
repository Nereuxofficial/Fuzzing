# Fuzzing

These are my personal Fuzzing projects. 

To use them install [hongfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs):
```
cargo install honggfuzz
```

and run this project via:
```
cargo hfuzz run [PROJECT_NAME]
```

It's also pretty easy to fuzz another project by replacing the library and 
adapting the src/main.rs. Though it should be noted that you should delete the 
folders hfuzz_workspace and hfuzz_target.
