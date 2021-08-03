pub use pallet::*;

pub mod benchmarking;
#[cfg(test)]
mod mock;
use xcm::v0::MultiAsset;

// TODO: make this instanciable.
#[frame_support::pallet]
pub mod pallet {

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config + crate::Config {
		/// The type of `fungible` that is being used under the hood.
		///
		/// This is useful for testing and checking.
		type TransactAsset: frame_support::traits::fungible::Mutate<Self::AccountId>;

		/// Give me a fungible asset that your asset transactor is going to accept.
		fn get_multi_asset() -> xcm::v0::MultiAsset;
	}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);
}