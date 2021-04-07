use crate::client::requester::Requester;
use crate::ext::context::Context;

// Implements itself into the client
// Will add a HashMap that links a ref to itself so it can handle launch_event
pub trait Extension {
    fn implement(&self, implementer: Requester) {}
    fn launch_event(&self, initiator: Requester, event: String, context: Context);
}