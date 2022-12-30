#![cfg(feature = "runtime-benchmarks")]

use crate as ClubModule;
use crate::*;
use frame_benchmarking::{account, benchmarks, vec};
use frame_system::RawOrigin;

benchmarks! {

	add_new_club {
		let club_id = 123;
		let name_len = 50;	// benchmark name_len?
	}: _(RawOrigin::Root, club_id, vec![0u8; name_len])
	verify {
		assert!(ClubModule::Pallet::<T>::clubs(club_id).is_some());
	}

	add_member {
		let club_id = 123;
		let name_len = 50;
		let member: T::AccountId = account("member", 0, 0);
		ClubModule::Pallet::<T>::add_new_club(RawOrigin::Root.into(), club_id, vec![0u8; name_len])?;
	}: _(RawOrigin::Root, club_id, member.clone())
	verify {
		assert!(ClubModule::Pallet::<T>::membership(club_id, member));
	}

	remove_club {
		let s in 0 .. 100000;
		let club_id = 123;
		let name_len = 50;
		ClubModule::Pallet::<T>::add_new_club(RawOrigin::Root.into(), club_id, vec![0u8; name_len])?;

		for i in 0..s {
			let member: T::AccountId = account("member", i, 0);
			ClubModule::Pallet::<T>::add_member(RawOrigin::Root.into(), club_id, member)?;
		}
		assert_eq!(ClubModule::Pallet::<T>::clubs(club_id).unwrap().members_count, s);
	}: _(RawOrigin::Root, club_id)
	verify {
		assert!(ClubModule::Pallet::<T>::clubs(club_id).is_none());
	}

	remove_member {
		let club_id = 123;
		let name_len = 50;
		let member: T::AccountId = account("member", 0, 0);
		ClubModule::Pallet::<T>::add_new_club(RawOrigin::Root.into(), club_id, vec![0u8; name_len])?;
		ClubModule::Pallet::<T>::add_member(RawOrigin::Root.into(), club_id, member.clone())?;
	}: _(RawOrigin::Root, club_id, member.clone())
	verify {
		assert!(!ClubModule::Pallet::<T>::membership(club_id, member));
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test)
}
