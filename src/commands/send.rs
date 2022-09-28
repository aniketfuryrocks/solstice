#[derive(clap::Args, Debug)]
pub struct SendCommand {
    from: String,
    to: String,
    ammont: String,
}

impl SendCommand {
    pub fn execute(&self) {
        println!("Sending SOL {:?}", self);
    }
}
