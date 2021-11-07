use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

#[test]
fn add_user_to_observed_user_list() {
	new_test_ext().execute_with(|| {
		let origin_account_id: u64 = 1;
		let target_address_id: u64 = 2;

		// Observe a single user.
		assert_ok!(Board::observe_user(Origin::signed(origin_account_id), target_address_id));

		// Check if user is observed.
		assert_eq!(Board::observing(origin_account_id).first(), Some(&target_address_id));
	});
}

#[test]
fn add_post_to_user_post_list() {
	new_test_ext().execute_with(|| {
		let origin_account_id: u64 = 1;

		assert_ok!(Board::create_post(
			Origin::signed(origin_account_id),
			"test message".as_bytes().to_vec(),
		));

		assert_eq!(Board::all_author_posts(origin_account_id).len(), 1);
	});
}

#[test]
fn user_cannot_unobserve_user_that_is_not_observed() {
	new_test_ext().execute_with(|| {
		let origin_account_id: u64 = 1;

		assert_noop!(
			Board::unobserve_user(Origin::signed(origin_account_id), 2),
			Error::<Test>::CannotUnobserveUserThatIsNotObserved
		);
	})
}

#[test]
fn user_cannot_remove_post_that_does_not_exist() {
	new_test_ext().execute_with(|| {
		let origin_account_id: u64 = 1;

		assert_noop!(
			Board::remove_post(Origin::signed(origin_account_id), H256::from_low_u64_be(1)),
			Error::<Test>::CannotRemovePostThatDoesNotExist
		);
	})
}

#[test]
fn user_can_remove_post_that_he_created() {
	new_test_ext().execute_with(|| {
		let origin_account_id: u64 = 1;
		assert_ok!(Board::create_post(
			Origin::signed(origin_account_id),
			"test message".as_bytes().to_vec(),
		));

		let post_id = Board::all_author_posts(origin_account_id).clone().first().unwrap().clone();

		assert_eq!(Board::all_author_posts(origin_account_id).len(), 1);

		assert_ok!(Board::remove_post(Origin::signed(origin_account_id), post_id));

		assert_eq!(Board::all_author_posts(origin_account_id).len(), 0);
	});
}

#[test]
fn user_can_comment_other_user_post() {
	new_test_ext().execute_with(|| {
		let author_post_account_id = 1;
		let commenter_account_id = 2;

		assert_ok!(Board::create_post(
			Origin::signed(author_post_account_id),
			"test message".as_bytes().to_vec()
		));

		let post_id =
			Board::all_author_posts(author_post_account_id).clone().first().unwrap().clone();

		assert_ok!(Board::comment_post(
			Origin::signed(commenter_account_id),
			"test comment".as_bytes().to_vec(),
			post_id
		));

		let comment_id = Board::post_comments(post_id).clone().first().unwrap().clone();

		assert_eq!(Board::post_comments(post_id), vec![comment_id]);
		assert_eq!(Board::all_comments(comment_id).unwrap().post_id, post_id);
	});
}

#[test]
fn user_cannot_comment_post_that_does_not_exists() {
	new_test_ext().execute_with(|| {
		let commenter_account_id = 1;
		let not_existing_post_id = 1;

		assert_noop!(
			Board::comment_post(
				Origin::signed(commenter_account_id),
				"test comment".as_bytes().to_vec(),
				H256::from_low_u64_be(not_existing_post_id)
			),
			Error::<Test>::CannotCommentPostThatDoesDoesNotExist
		);
	})
}
