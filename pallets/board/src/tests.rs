use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn add_user_to_observed_user_list() {
	new_test_ext().execute_with(|| {
		let origin_account_id: u64 = 1;
		let target_address_id: u64 = 2;

		// Observe a single user.
		assert_ok!(TemplateModule::observe_user(
			Origin::signed(origin_account_id),
			target_address_id
		));

		// Check if user is observed.
		assert_eq!(TemplateModule::observing(origin_account_id).first(), Some(&target_address_id));
	});
}

#[test]
fn add_post_to_user_post_list() {
	new_test_ext().execute_with(|| {
		let origin_account_id: u64 = 1;

		assert_ok!(TemplateModule::create_post(
			Origin::signed(origin_account_id),
			"test message".as_bytes().to_vec(),
		));

		assert_eq!(TemplateModule::all_author_posts(origin_account_id).len(), 1);
		// Ensure the expected error is thrown when no value is present.
		// assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn user_cannot_unobserve_user_that_is_not_observed() {
	new_test_ext().execute_with(|| {
		let origin_account_id: u64 = 1;

		assert_noop!(
			TemplateModule::unobserve_user(Origin::signed(origin_account_id), 2),
			Error::<Test>::CannotUnobserveUserThatIsNotObserved
		);
	})
}
