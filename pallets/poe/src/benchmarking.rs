//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
//use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use frame_support::{BoundedVec, pallet_prelude::Get};
use sp_std::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_claim(b: Linear<1, {T::MaxClaimLength::get()}>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        
    }

    fn do_something() {
        let value = 100u32.into();
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        do_something(RawOrigin::Signed(caller), value);

        assert_eq!(Something::<T>::get(), Some(value));
    }

    #[benchmark]
    fn cause_error() {
        Something::<T>::put(100u32);
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        cause_error(RawOrigin::Signed(caller));

        assert_eq!(Something::<T>::get(), Some(101u32));
    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
