use BitterHarmony;

#[test]
fn it_works() {
    let mut x = BitterHarmony::client::requester::RequesterBuilder::new()
        .add_setting("token".to_owned(), "something".to_owned())
        .start();
}
