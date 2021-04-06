use crate::Client::requester::Requester;
use crate::Ext::context::Context;

// Implements itself into the client
// Will add a HashMap that links a ref to itself so it can handle launch_event
pub trait Extension {
    fn implement(&self, &mut implementer: Requester) {
        implementer.extensions.insert() //;
    };
    fn launch_event(&self, initiator: Requester, event: String, context: Context);
}