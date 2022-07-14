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
	pub struct Kitty<T: Config> {
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
	#[pallet::getter(fn kitty_cnt)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type KittyCnt<T> = StorageValue<_, Id, ValueQuery>;

	// key: id value: Kitties
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, Id, Kitty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn kitties_owned)]
	/// Keeps track of what accounts own what Kitty.
	pub(super) type KittiesOwned<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Id>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyStored(Vec<u8>, u32),
		Transferred(T::AccountId, T::AccountId, u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Handles arithemtic overflow when incrementing the Kitty counter.
		KittyCntOverflow,
		/// An account cannot own more Kitties than `MaxKittyCount`.
		ExceedMaxKittyOwned,
		/// Buyer cannot be the owner.
		BuyerIsKittyOwner,
		/// Cannot transfer a kitty to its owner.
		TransferToSelf,
		/// Handles checking whether the Kitty exists.
		KittyNotExist,
		/// Handles checking that the Kitty is owned by the account transferring, buying or setting a price for it.
		NotKittyOwner,
		/// Ensures the Kitty is for sale.
		KittyNotForSale,
		/// Ensures that the buying price is greater than the asking price.
		KittyBidPriceTooLow,
		/// Ensures that an account has enough funds to purchase a Kitty.
		NotEnoughBalance,
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
		pub fn create_kitty(origin: OriginFor<T>, dna:Vec<u8>, price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.

			let gender = Self::gen_gender(dna.clone())?;
			let kitty = Kitty {
				dna: dna.clone(),
				owner: who.clone(),
				price: price,
				gender: gender,
			};

			let mut new_cnt = KittyCnt::<T>::get();


			<KittiesOwned<T>>::mutate(&who, |kitty_vec| {
				kitty_vec.push(new_cnt)
			});

			<Kitties<T>>::insert(new_cnt, kitty);
			new_cnt +=1;
			<KittyCnt<T>>::put(new_cnt);


			// Emit an event.
			Self::deposit_event(Event::KittyStored(dna,price));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer_kitty(origin: OriginFor<T>,who_to: T::AccountId, kitty_id:u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who_from = ensure_signed(origin)?;

			// check xem mao co dung chu ko.
			ensure!(Self::is_kitty_owner(&kitty_id, &who_from)?, <Error<T>>::NotKittyOwner);
			// check xem chu cu va chu moi co phai la mot khong
			ensure!(who_from != who_to, <Error<T>>::TransferToSelf);
			let to_owned = <KittiesOwned<T>>::get(&who_to);
			Self::transfer_kitty_to(&kitty_id, &who_to)?;

			Self::deposit_event(Event::Transferred(who_from, who_to, kitty_id));
			Ok(())
		}

	}
	
	// helper function

	impl<T: Config> Pallet<T> {
		fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
			let mut res = Gender::Male;
			if dna.len() % 2 != 0 {
				res = Gender::Female;
			}

			Ok(res)
		}

		pub fn is_kitty_owner(kitty_id: &u32, acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty.owner == *acct),
				None => Err(<Error<T>>::KittyNotExist)
			}
		}

		
		// #[transactional]
		pub fn transfer_kitty_to(
			kitty_id: &u32,
			who_to: &T::AccountId,
		) -> Result<(), Error<T>> {
			let mut kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExist)?;

			let prev_owner = kitty.owner.clone();

			// Remove `kitty_id` from the KittyOwned vector of `prev_kitty_owner`
			<KittiesOwned<T>>::mutate(&prev_owner, |owned| {
				if let Some(id) = owned.iter().position(|&id| id == *kitty_id) {
					owned.swap_remove(id);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::KittyNotExist)?;

			// Update the kitty owner
			kitty.owner = who_to.clone();


			<Kitties<T>>::insert(kitty_id, kitty);

			<KittiesOwned<T>>::mutate(who_to, |vec| {
				vec.push(*kitty_id)
			});

			Ok(())
		}
	}
}
