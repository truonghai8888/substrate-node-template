//! Benchmarking setup for pallet-Kitties

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;


benchmarks! {
	// tên của benchmark
	create_kitty {
		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty (RawOrigin::Signed(caller))

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa 
	verify {
		assert_eq!(CountForKitties::<T>::get(), 1);
	}

	// tên của benchmark
	transfer {
		let caller: T::AccountId = whitelisted_caller();

		let dna = b"test".to_vec();
		let gender = Gender::Male;

		let _ = <Kitties<T>>::mint(&caller, dna.clone(), gender);
		// and reap this user.
		let to: T::AccountId = account("user", 0, 0);
		// let dna = &KittiesOwned::<T>::get(&caller)[0]?;
		
	}: transfer (RawOrigin::Signed(caller), to, dna.to_vec())

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa 
	verify {
		assert_eq!(CountForKitties::<T>::get(), 1);
	}

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
