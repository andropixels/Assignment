use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_root() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(MembersModule::set_root(Origin::signed(1)));
		// Read pallet storage and assert an expected result.
		// assert_eq!(TemplateMembers::something(), Some(42));
	});
}

#[test]
fn add_member() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
        
		assert_noop!(MembersModule::add_member(Origin::signed(1),0), Error::<Test>::RootAlreadySet);
	});
}

#[test]
fn remove_member() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
        
		assert_noop!(MembersModule::remove_member(Origin::signed(1),0), Error::<Test>::RootAlreadySet);
	});
}
