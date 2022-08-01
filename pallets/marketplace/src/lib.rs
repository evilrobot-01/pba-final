#![cfg_attr(not(feature = "std"), no_std)]
use codec::HasCompact;
use frame_support::{
	dispatch::DispatchResult, pallet_prelude::*, traits::fungibles::Create,
	traits::fungibles::Inspect, traits::fungibles::Mutate,
	traits::tokens::fungibles::metadata::Mutate as MutateMetadata,
	traits::tokens::fungibles::Inspect as FungibleInspect,
	traits::tokens::nonfungibles::Inspect as NonFungibleInspect, traits::Currency,
	traits::ReservableCurrency, traits::Time, PalletId,
};
use frame_system::pallet_prelude::OriginFor;
use frame_system::pallet_prelude::*;
pub use pallet::*;
use pallet_dex::Swap;
use sp_runtime::{traits::AtLeast32BitUnsigned, traits::Bounded};
pub use types::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
mod types;

type AssetIdOf<T> =
	<<T as Config>::Assets as FungibleInspect<<T as frame_system::Config>::AccountId>>::AssetId;
type BalanceOf<T> =
	<<T as Config>::Assets as FungibleInspect<<T as frame_system::Config>::AccountId>>::Balance;
type CollectionIdOf<T> = <<T as Config>::Uniques as NonFungibleInspect<
	<T as frame_system::Config>::AccountId,
>>::CollectionId;
type ItemIdOf<T> =
	<<T as Config>::Uniques as NonFungibleInspect<<T as frame_system::Config>::AccountId>>::ItemId;
type PriceOf<T> =
	<<T as Config>::Assets as FungibleInspect<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// Identifier type for a fungible asset
		type AssetId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ PartialOrd;

		// Balance inspection for fungible assets
		type Assets: FungibleInspect<Self::AccountId, AssetId = Self::AssetId>;

		/// Identifier type for a collection of items
		type CollectionId: Member + Parameter + MaxEncodedLen + Copy;

		// Auto-swapping to facilitate buying/selling using any asset/token.
		type DEX: Swap<
			Self::AccountId,
			AssetId = Self::AssetId,
			Balance = <Self::Assets as FungibleInspect<
				<Self as frame_system::Config>::AccountId,
			>>::Balance,
		>;

		/// The type used to identify a unique item within a collection
		type ItemId: Member + Parameter + MaxEncodedLen + Copy;

		// Balance inspection for non-fungible assets
		type Uniques: NonFungibleInspect<
			Self::AccountId,
			CollectionId = Self::CollectionId,
			ItemId = Self::ItemId,
		>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Stores listings based on composite key of asset pair
	#[pallet::storage]
	pub(super) type LiquidityPools<T: Config> =
		StorageMap<_, Twox64Concat, (CollectionIdOf<T>, ItemIdOf<T>), Listing<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [collection, item, price]
		Listed(CollectionIdOf<T>, ItemIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// todo: document
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn list(
			origin: OriginFor<T>,
			collection: CollectionIdOf<T>,
			item: ItemIdOf<T>,
			price: PriceOf<T>,
			asset: AssetIdOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// todo: create/replace list of item

			todo!()
		}

		/// todo: document
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn delist(
			origin: OriginFor<T>,
			collection: CollectionIdOf<T>,
			item: ItemIdOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// todo: remove listing of item

			todo!()
		}

		/// todo: document
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn purchase(
			origin: OriginFor<T>,
			collection: CollectionIdOf<T>,
			item: ItemIdOf<T>,
			asset: AssetIdOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Lookup item listing
			let list_price = <BalanceOf<T>>::default();
			let balance = T::Assets::balance(asset, &who);

			let listing_asset = <AssetIdOf<T>>::default();
			// todo: conclude transfer of item, auto-swapping swapping between tokens if required (provided liquidity
			// pool exists and liquidity available)

			// todo: DEX needs functionality to be able to specify minimum quantity returned
			T::DEX::swap(list_price, listing_asset, asset, who)?;

			todo!()
		}
	}
}