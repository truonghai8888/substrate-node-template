#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;
use frame_support::dispatch::fmt;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitties<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
	}
	pub type Id = u32;

	#[derive(TypeInfo, Encode, Decode, Debug)]
	pub enum Gender {
		Male,
		Female
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type KittyId<T> = StorageValue<_, Id, ValueQuery>;

	// key: id value: Kitties
	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type Kitty<T: Config> = StorageMap<_, Blake2_128Concat, Id, Kitties<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Kittytored(Vec<u8>, u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		TooYoung,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_student(origin: OriginFor<T>, dna:Vec<u8>, price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.

			let gender = Self::gen_gender(dna.clone())?;
			let kitty = Kitties {
				dna: dna.clone(),
				owner: who,
				price: price,
				gender: gender,
			};

			let mut current_id = KittyId::<T>::get();

			Kitty::<T>::insert(current_id, kitty);
			current_id +=1;
			KittyId::<T>::put(current_id);
			// Emit an event.
			Self::deposit_event(Event::Kittytored(dna,price));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

	}
}


// helper function

impl<T> Pallet<T> {
	fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Male;
		if dna.len() % 2 != 0 {
			res = Gender::Female;
		}

		Ok(res)
	}
}
