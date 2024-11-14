use super::mock::*;
use frame_support::{assert_noop, assert_ok};   // assert_err
// use sp_runtime::ModuleError;
// use sp_runtime::DispatchError::Module;

type Error = crate::Error::<TestRuntime>;

#[test]
fn set_value_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(RuntimeOrigin::signed(ALICE), false));
		// OR: assert_ok!(Flipper::set_value(Origin::signed(ALICE), false));
		// in older versions.
		assert_eq!(Flipper::value(), Some(false));
	});
}

#[test]
fn set_value_err_already_set() {
	new_test_ext().execute_with(|| {
		// This solution gets the work done but is UGLY:
		// assert_ok!(Flipper::set_value(RuntimeOrigin::signed(ALICE), true));
		// // println!("\n\n{:?}\n\n", Flipper::set_value(RuntimeOrigin::signed(ALICE), false));		
		// assert_err!(
		// 	Flipper::set_value(RuntimeOrigin::signed(ALICE), false),
		// 	Module(ModuleError { index: 1, error: [1, 0, 0, 0], message: Some("AlreadySet") })
		// );

		// The "official" solution:
		assert_ok!(Flipper::set_value(RuntimeOrigin::signed(1), true));
		assert_noop!(
			Flipper::set_value(RuntimeOrigin::signed(1), true),
			// RuntimeOrigin::<TestRuntime>::AlreadySet
			Error::AlreadySet
		);
	});
}

#[test]
fn flip_value_ok() {
	new_test_ext()
		.execute_with(|| {
			assert_ok!(Flipper::set_value(RuntimeOrigin::signed(ALICE), true));
			assert_eq!(Flipper::value(), Some(true));
			assert_ok!(Flipper::flip_value(RuntimeOrigin::signed(ALICE)));
			assert_eq!(Flipper::value(), Some(false));
		});
}

#[test]
fn flip_value_ko() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Flipper::flip_value(RuntimeOrigin::signed(1)),   // value not set
			// crate::Error::<TestRuntime>::NoneValue
			Error::NoneValue
		);
	});
}
