pub fn mod_a_mod_b_fn() {
//    let () = {
//        use mock_trait::{MockResult, MockTrait};
//        match mod_a_mod_b_fn.call_mock(()) {
//            MockResult::Continue(()) => (),
//            MockResult::Return(()) => return (),
//        }
//    };
    use mock_trait::{MockResult, MockTrait};
    let result = {
        use mock_trait::{MockResult, MockTrait};
        //    mock_trait::MockTrait::call_mock(mod_a_mod_b_fn, ());
        mod_a_mod_b_fn.call_mock(());
//        println!("Hello mod a mod b fn!");
    };
}

fn ignoring(arg_1: u32, mut arg_2: i32, mut arg_3: &i32, _: f64) {
    assert!(arg_1 == 0);
}
