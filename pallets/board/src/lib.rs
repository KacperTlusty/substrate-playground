#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResult, inherent::Vec, pallet_prelude::*, sp_runtime::traits::Hash,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type MaxObserversPerUser: Get<u32>;

		#[pallet::constant]
		type MaxPostsPerUser: Get<u32>;
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Post<T: Config> {
		pub author: AccountOf<T>,
		pub message: Vec<u8>,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn all_posts)]
	pub(super) type AllPosts<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Post<T>>;

	#[pallet::storage]
	#[pallet::getter(fn all_author_posts)]
	pub(super) type AllAuthorPosts<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<T::Hash, T::MaxPostsPerUser>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn observing)]
	pub(super) type Observing<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<T::AccountId, T::MaxObserversPerUser>,
		ValueQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// User created new post on their board. [author, post_id]
		PostCreated(T::AccountId, T::Hash),
		UserObserved(T::AccountId, T::AccountId),
		UserUnobserved(T::AccountId, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ExceedMaxPostsPerUser,
		/// User cannot observe more users than allowed limit.
		ExceedMaxObserversPerUser,
		/// User cannot unobserve user that he is not observing.
		CannotUnobserveUserThatIsNotObserved,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100)]
		pub fn create_post(origin: OriginFor<T>, content: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let post_id = Self::mint_post(&sender, content)?;

			log::info!("A new post have been created with ID: {:?} author {:?}.", post_id, sender);

			Self::deposit_event(Event::PostCreated(sender, post_id));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn observe_user(origin: OriginFor<T>, user_to_observe: T::AccountId) -> DispatchResult {
			let user_id = ensure_signed(origin)?;
			<Observing<T>>::try_mutate(&user_id, |observing_vec| {
				observing_vec.try_push(user_to_observe.clone())
			})
			.map_err(|_| <Error<T>>::ExceedMaxObserversPerUser)?;

			log::info!("An user {:?} starting to observe user {:?}.", user_id, user_to_observe);

			Self::deposit_event(Event::UserObserved(user_id, user_to_observe));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn unobserve_user(
			origin: OriginFor<T>,
			user_to_unobserve: T::AccountId,
		) -> DispatchResult {
			let user_id = ensure_signed(origin)?;

			<Observing<T>>::try_mutate(&user_id, |observing_vec| {
				if let Some(index) = observing_vec.iter().position(|id| id == &user_id) {
					observing_vec.swap_remove(index);
					return Ok(());
				}
				Err(())
			})
			.map_err(|_| <Error<T>>::CannotUnobserveUserThatIsNotObserved)?;

			log::info!("An user {:?} unobserved user {:?}.", user_id, user_to_unobserve);

			Self::deposit_event(Event::UserUnobserved(user_id, user_to_unobserve));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn mint_post(author: &T::AccountId, message: Vec<u8>) -> Result<T::Hash, Error<T>> {
			let post = Post::<T> { message, author: author.clone() };

			let post_id = T::Hashing::hash_of(&post);

			<AllPosts<T>>::insert(post_id, post);
			<AllAuthorPosts<T>>::try_mutate(&author, |posts_vec| posts_vec.try_push(post_id))
				.map_err(|_| <Error<T>>::ExceedMaxPostsPerUser)?;

			Ok(post_id)
		}
	}
}
