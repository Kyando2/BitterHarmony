use bitter_harmony;
use std::{thread, time};

#[test]
fn it_works() {
    let mut gate = bitter_harmony::gateway::runner::GatePool::new(3);
    gate.connect();
    thread::sleep(time::Duration::from_millis(2000));
}
