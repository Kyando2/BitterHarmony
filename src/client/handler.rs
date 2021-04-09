use crate::event::Context;

pub trait BitterHandler {
    fn on_message(&Self, ctx: Context);
    fn on_command(&Self, ctx: Context);
}