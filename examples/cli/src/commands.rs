pub const COMMANDS: [&Command; 1] = [&Command {
    name: "find-controllers",
    func: find_controllers,
}];

pub struct Command {
    pub name: &'static str,
    pub func: fn(),
}

impl Command {
    pub fn exec(&self) {
        println!("{}", self.name);
        (self.func)();
    }
}

fn find_controllers() {
    println!();
    println!("------ >> find controllers");
    println!();
}
