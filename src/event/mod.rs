pub mod context;
pub use context::Context;

pub enum Event {
    Message{ctx: Context},
    Command{ctx: Context}
}
