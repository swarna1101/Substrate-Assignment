#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::WeightInfo;
	use frame_support::{inherent::Vec, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;

		/// The maximum length a name may be.
		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	type ClubId = u32;

	#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct ClubDetails<T: Config> {
		pub id: ClubId,
		pub members_count: u32,
		pub name: BoundedVec<u8, T::MaxLength>,
	}

	// ClubId -> Option<ClubDetails>
	#[pallet::storage]
	#[pallet::getter(fn clubs)]
	pub type Clubs<T> = StorageMap<_, Blake2_128Concat, ClubId, ClubDetails<T>, OptionQuery>;

	// (ClubId, AccountId) -> bool
	#[pallet::storage]
	#[pallet::getter(fn membership)]
	pub type Membership<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		ClubId,
		Blake2_128Concat,
		T::AccountId,
		bool,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// [club_id, club_name]
		ClubAdded(ClubId, Vec<u8>),
		/// [club_id]
		ClubRemoved(ClubId),
		/// [club_id, member_id]
		ClubMemberAdded(ClubId, T::AccountId),
		/// [club_id, member_id]
		ClubMemberRemoved(ClubId, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		StorageOverflow,
		NameTooLong,
		DuplicateClubId,
		ClubNotFound,
		MemberNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::add_new_club())]
		pub fn add_new_club(
			origin: OriginFor<T>,
			club_id: ClubId,
			name: Vec<u8>,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(Self::clubs(club_id).is_none(), Error::<T>::DuplicateClubId);

			let club = ClubDetails {
				id: club_id,
				members_count: 0,
				name: name.clone().try_into().map_err(|()| Error::<T>::NameTooLong)?,
			};

			Clubs::<T>::insert(&club_id, &club);
			Self::deposit_event(Event::<T>::ClubAdded(club_id, name));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::remove_club(Pallet::<T>::get_club_members_count(*club_id)))]
		pub fn remove_club(origin: OriginFor<T>, club_id: ClubId) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(Self::clubs(club_id).is_some(), Error::<T>::ClubNotFound);

			let sz = Self::clubs(&club_id).unwrap().members_count;
			let res = Membership::<T>::clear_prefix(&club_id, sz, None);
			ensure!(res.maybe_cursor == None, Error::<T>::StorageOverflow);

			Clubs::<T>::remove(&club_id);
			Self::deposit_event(Event::<T>::ClubRemoved(club_id));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::add_member())]
		pub fn add_member(
			origin: OriginFor<T>,
			club_id: ClubId,
			member: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(Self::clubs(club_id).is_some(), Error::<T>::ClubNotFound);

			if !Self::membership(&club_id, &member) {
				let mut club = Self::clubs(&club_id).unwrap();
				club.members_count =
					club.members_count.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;

				Clubs::<T>::insert(&club_id, &club);
				Membership::<T>::insert(&club_id, &member, &true);
				Self::deposit_event(Event::<T>::ClubMemberAdded(club_id, member));
			}
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::remove_member())]
		pub fn remove_member(
			origin: OriginFor<T>,
			club_id: ClubId,
			member: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(Self::clubs(club_id).is_some(), Error::<T>::ClubNotFound);
			ensure!(Self::membership(&club_id, &member), Error::<T>::MemberNotFound);

			let mut club = Self::clubs(&club_id).unwrap();
			club.members_count -= 1;

			Clubs::<T>::insert(&club_id, &club);
			Membership::<T>::remove(&club_id, &member);
			Self::deposit_event(Event::<T>::ClubMemberRemoved(club_id, member));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn get_all_clubs() -> Vec<ClubId> {
			Clubs::<T>::iter_keys().collect()
		}

		pub fn get_all_members(club_id: &ClubId) -> Vec<T::AccountId> {
			Membership::<T>::iter_key_prefix(&club_id).collect()
		}

		fn get_club_members_count(club_id: ClubId) -> u32 {
			match Self::clubs(club_id) {
				None => 0,
				Some(club) => club.members_count,
			}
		}
	}
}
