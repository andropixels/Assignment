//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Members;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	set_root {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Root::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(Members, crate::mock::new_test_ext(), crate::mock::Test);
}



