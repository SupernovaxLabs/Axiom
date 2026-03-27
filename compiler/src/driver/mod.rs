pub struct CompilerDriver;

impl CompilerDriver {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!("axiomc bootstrap: compiler workspace is initialized");
        Ok(())
    }
}
