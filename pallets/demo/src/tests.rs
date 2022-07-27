use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(DemoModule::create_student(Origin::signed(1), b"haiTX".to_vec(), 22));
		// Read pallet storage and assert an expected result.
		assert_eq!(DemoModule::student_id(), 1);
	});
}

#[test]
fn error_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_noop!(DemoModule::create_student(Origin::signed(1), [1,2,3].to_vec(), 12), Error::<Test>::TooYoung);
		// Read pallet storage and assert an expected result.
		assert_eq!(DemoModule::student_id(), 0);
	});
}
