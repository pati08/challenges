#[derive(clap::Parser)]
pub struct Args {
    /// The day number to run
    #[arg(short, long)]
    day: u8,
    /// Whether to run part a only
    #[arg(short, long)]
    a_only: bool,
}

pub struct ProblemDesc;
impl challenges_input::ProblemDesc for ProblemDesc {
    type Args = Args;
    const TRIM: bool = false;
}
