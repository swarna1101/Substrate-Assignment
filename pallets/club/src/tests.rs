use crate as pallet_club;
use crate::{mock::*, Error};
use frame_support::{assert_err, assert_ok, error::BadOrigin};

#[test]
fn test_add_club() {
	new_test_ext().execute_with(|| {
		// 1. Add a new club
		let name = "Real Madrid".as_bytes().to_vec();
		assert_ok!(ClubModule::add_new_club(Origin::root(), 1, name.clone()));

		// 2. Verify event is emitted
		let events = System::events();
		events
			.iter()
			.find(|r| Event::ClubModule(pallet_club::Event::ClubAdded(1, name.clone())) == r.event)
			.expect("ClubAdded event not found");

		// 3. Verify storage is updated properly
		let club = ClubModule::clubs(1).expect("Club with id:{1} should exist");

		assert_eq!(club.id, 1);
		assert_eq!(club.members_count, 0);
		assert_eq!(club.name.to_vec(), name);

		// 4. Try adding club with duplicate id (should fail)
		assert_err!(
			ClubModule::add_new_club(Origin::root(), 1, name),
			Error::<Test>::DuplicateClubId
		);
	})
}

#[test]
fn test_add_member() {
	new_test_ext().execute_with(|| {
		// 1. Add a club
		let name = "Real Madrid".as_bytes().to_vec();
		assert_ok!(ClubModule::add_new_club(Origin::root(), 1, name.clone()));

		assert_eq!(ClubModule::membership(1, 001), false);
		assert_eq!(ClubModule::clubs(1).unwrap().members_count, 0);

		// 2. Verify remove member works
		assert_ok!(ClubModule::add_member(Origin::root(), 1, 001));
		assert_eq!(ClubModule::membership(1, 001), true);
		assert_eq!(ClubModule::clubs(1).unwrap().members_count, 1);

		// 3. Verify event is emitted
		let events = System::events();
		events
			.iter()
			.find(|r| Event::ClubModule(pallet_club::Event::ClubMemberAdded(1, 001)) == r.event)
			.expect("ClubMemberAdded event not found");
	})
}

#[test]
fn test_remove_club() {
	new_test_ext().execute_with(|| {
		// 1. Add a club
		let name = "Real Madrid".as_bytes().to_vec();
		assert_ok!(ClubModule::add_new_club(Origin::root(), 1, name.clone()));
		assert_ok!(ClubModule::add_member(Origin::root(), 1, 001));

		// 2. Verify removing club works
		assert_ok!(ClubModule::remove_club(Origin::root(), 1));
		assert!(ClubModule::clubs(1).is_none());
		assert_eq!(ClubModule::get_all_members(&1), vec![] as Vec<u64>);

		// 3. Verify event is emitted
		let events = System::events();
		events
			.iter()
			.find(|r| Event::ClubModule(pallet_club::Event::ClubRemoved(1)) == r.event)
			.expect("ClubRemoved event not found");

		// 4. Try removing non-existant club
		assert_err!(ClubModule::remove_club(Origin::root(), 1), Error::<Test>::ClubNotFound);
	})
}

#[test]
fn test_remove_member() {
	new_test_ext().execute_with(|| {
		// 1. Add a club & member
		let name = "Real Madrid".as_bytes().to_vec();
		assert_ok!(ClubModule::add_new_club(Origin::root(), 1, name.clone()));
		assert_ok!(ClubModule::add_member(Origin::root(), 1, 001));

		// 2. Verify removing member works
		assert_ok!(ClubModule::remove_member(Origin::root(), 1, 001));
		assert_eq!(ClubModule::membership(1, 001), false);
		assert_eq!(ClubModule::clubs(1).unwrap().members_count, 0);

		// 3. Verify event is emitted
		let events = System::events();
		events
			.iter()
			.find(|r| Event::ClubModule(pallet_club::Event::ClubMemberRemoved(1, 001)) == r.event)
			.expect("ClubMemberRemoved event not found");

		// 4. Try removing member from non-existant club
		assert_err!(ClubModule::remove_member(Origin::root(), 2, 001), Error::<Test>::ClubNotFound);

		// 5. Try removing non-existant member from a club
		assert_err!(
			ClubModule::remove_member(Origin::root(), 1, 001),
			Error::<Test>::MemberNotFound
		);
	})
}

#[test]
fn test_root_privilege() {
	new_test_ext().execute_with(|| {
		let name = "Real Madrid".as_bytes().to_vec();

		// Calls with ROOT origin
		assert_ok!(ClubModule::add_new_club(Origin::root(), 1, name.clone()));
		assert_ok!(ClubModule::add_member(Origin::root(), 1, 001));
		assert_ok!(ClubModule::remove_member(Origin::root(), 1, 001));
		assert_ok!(ClubModule::remove_club(Origin::root(), 1));

		// Calls with SIGNED origin
		assert_err!(ClubModule::add_new_club(Origin::signed(1), 1, name.clone()), BadOrigin);
		assert_err!(ClubModule::add_member(Origin::signed(1), 1, 001), BadOrigin);
		assert_err!(ClubModule::remove_member(Origin::signed(1), 1, 001), BadOrigin);
		assert_err!(ClubModule::remove_club(Origin::signed(1), 1), BadOrigin);
	})
}

#[test]
fn test_get_all_clubs() {
	new_test_ext().execute_with(|| {
		// 1. Add few clubs
		let name = "Real Madrid".as_bytes().to_vec();
		assert_ok!(ClubModule::add_new_club(Origin::root(), 1, name.clone()));
		assert_ok!(ClubModule::add_new_club(Origin::root(), 2, name.clone()));
		assert_ok!(ClubModule::add_new_club(Origin::root(), 3, name.clone()));

		// 2. Get all club ids
		let mut clubs = ClubModule::get_all_clubs();
		clubs.sort();
		assert_eq!(clubs, vec![1, 2, 3]);
	})
}

#[test]
fn test_get_all_members() {
	new_test_ext().execute_with(|| {
		// 1. Add few clubs
		let name = "Real Madrid".as_bytes().to_vec();
		assert_ok!(ClubModule::add_new_club(Origin::root(), 1, name.clone()));
		assert_ok!(ClubModule::add_new_club(Origin::root(), 2, name.clone()));

		// 2. Add few members
		assert_ok!(ClubModule::add_member(Origin::root(), 1, 001));
		assert_ok!(ClubModule::add_member(Origin::root(), 1, 002));

		assert_ok!(ClubModule::add_member(Origin::root(), 2, 001));
		assert_ok!(ClubModule::add_member(Origin::root(), 2, 003));

		// 2. Get all club members
		let mut club1_members = ClubModule::get_all_members(&1);
		let mut club2_members = ClubModule::get_all_members(&2);

		club1_members.sort();
		club2_members.sort();

		assert_eq!(club1_members, vec![001, 002]);
		assert_eq!(club2_members, vec![001, 003]);
	})
}
