# Fuzzing

These are my personal Fuzzing projects. 

To use them install [hongfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs):
```
cargo install honggfuzz
```

Then go into a project folder:
```bash
cd [PROJECT_NAME]
```

and run this project via:
```
cargo hfuzz run fuzzing
```

It's also pretty easy to fuzz another project by replacing the library and 
adapting the src/main.rs. Though it should be noted that you should delete the 
folders hfuzz_workspace and hfuzz_target.