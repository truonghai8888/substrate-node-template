#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// use frame_support::pallet_prelude::*;
use frame_support::{
	pallet_prelude::*,
	traits::{tokens::ExistenceRequirement, Currency, Randomness},
};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;
use sp_runtime::ArithmeticError;
use scale_info::TypeInfo;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	pub struct Kitty<T: Config> {
		// Using 16 bytes to represent a kitty DNA
		pub dna: Vec<u8>,
		// `None` assumes not for sale
		pub price: Option<BalanceOf<T>>,
		pub gender: Gender,
		pub owner: T::AccountId,
	}
	pub type Id = u32;
	pub type Dna = Vec<u8>;

	// Set Gender type in kitty struct
	#[derive(TypeInfo, Encode, Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Currency handler for the kitties pallet.
		type Currency: Currency<Self::AccountId>;
	}

	// Errors
	#[pallet::error]
	pub enum Error<T> {
		/// An account may only own `MaxKittiesOwned` kitties.
		TooManyOwned,
		/// Trying to transfer or buy a kitty from oneself.
		TransferToSelf,
		/// This kitty already exists!
		DuplicateKitty,
		/// This kitty does not exist!
		NoKitty,
		/// You are not the owner of this kitty.
		NotOwner,
		/// This kitty is not for sale.
		NotForSale,
		/// Ensures that the buying price is greater than the asking price.
		BidPriceTooLow,
		/// You need to have two cats with different gender to breed.
		CantBreed,
	}

	// Events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new kitty was successfully created.
		Created { kitty: Vec<u8>, owner: T::AccountId },
		/// The price of a kitty was successfully set.
		PriceSet { kitty: Vec<u8>, price: Option<BalanceOf<T>> },
		/// A kitty was successfully transferred.
		Transferred { from: T::AccountId, to: T::AccountId, kitty: Vec<u8> },
		/// A kitty was successfully sold.
		Sold { seller: T::AccountId, buyer: T::AccountId, kitty: Vec<u8>, price: BalanceOf<T> },
	}

	/// Keeps track of the number of kitties in existence.
	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Maps the kitty struct to the kitty DNA.
	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>>;

	/// Track the kitties owned by each account.
	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Vec<Vec<u8>>,
		ValueQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new unique kitty.
		///
		/// The actual kitty creation is done in the `mint()` function.
		#[pallet::weight(0)]
		pub fn create_kitty(origin: OriginFor<T>, dna:Vec<u8>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let who = ensure_signed(origin)?;

			log::info!("total balance: {:?}", T::Currency::total_balance(&who));

			let gender = Self::gen_gender(dna.clone())?;
			// Write new kitty to storage by calling helper function
			Self::mint(&who, dna, gender)?;

			Ok(())
		}

		/// Directly transfer a kitty to another recipient.
		///
		/// Any account that holds a kitty can send it to another Account. This will reset the
		/// asking price of the kitty, marking it not for sale.
		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			dna: Vec<u8>,
		) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let from = ensure_signed(origin)?;
			let kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NoKitty)?;
			ensure!(kitty.owner == from, Error::<T>::NotOwner);
			Self::do_transfer(dna, to)?;
			Ok(())
		}
	}

	//** Our helper functions.**//

	impl<T: Config> Pallet<T> {
		fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
			let mut res = Gender::Male;
			if dna.len() % 2 != 0 {
				res = Gender::Female;
			}

			Ok(res)
		}

		// Helper to mint a kitty
		pub fn mint(
			owner: &T::AccountId,
			dna: Vec<u8>,
			gender: Gender,
		) -> Result<Vec<u8>, DispatchError> {
			// Create a new object
			let kitty = Kitty::<T> { dna: dna.clone(), price: None, gender, owner: owner.clone() };

			// Check if the kitty does not already exist in our storage map
			ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

			// Performs this operation first as it may fail
			let count = CountForKitties::<T>::get();
			let new_count = count.checked_add(1).ok_or(ArithmeticError::Overflow)?;

			// Append kitty to KittiesOwned
			KittiesOwned::<T>::append(&owner, kitty.dna.clone());

			// Write new kitty to storage
			Kitties::<T>::insert(kitty.dna.clone(), kitty);
			CountForKitties::<T>::put(new_count);

			// Deposit our "Created" event.
			Self::deposit_event(Event::Created { kitty: dna.clone(), owner: owner.clone() });

			// Returns the DNA of the new kitty if this succeeds
			Ok(dna)
		}

		// Update storage to transfer kitty
		pub fn do_transfer(
			dna: Vec<u8>,
			to: T::AccountId,
		) -> DispatchResult {
			// Get the kitty
			let mut kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NoKitty)?;
			let from = kitty.owner;

			ensure!(from != to, Error::<T>::TransferToSelf);
			let mut from_owned = KittiesOwned::<T>::get(&from);

			// Remove kitty from list of owned kitties.
			if let Some(ind) = from_owned.iter().position(|id| *id == dna) {
				from_owned.swap_remove(ind);
			} else {
				return Err(Error::<T>::NoKitty.into());
			}

			// Add kitty to the list of owned kitties.
			let mut to_owned = KittiesOwned::<T>::get(&to);
			to_owned.push(dna.clone());

			// Transfer succeeded, update the kitty owner and reset the price to `None`.
			kitty.owner = to.clone();
			kitty.price = None;

			// Write updates to storage
			Kitties::<T>::insert(&dna, kitty);
			KittiesOwned::<T>::insert(&to, to_owned);
			KittiesOwned::<T>::insert(&from, from_owned);

			Self::deposit_event(Event::Transferred { from, to, kitty: dna });

			Ok(())
		}
	}
}
