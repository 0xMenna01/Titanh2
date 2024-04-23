// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

mod types;
pub use types::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use common_types::{Balance, Time};
	use frame_support::{
		pallet_prelude::{StorageDoubleMap, *},
		Blake2_128Concat,
	};
	use frame_system::pallet_prelude::*;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type in which we record balances
		type Balance: Balance;
		/// Type for managing time
		type Timestamp: Time;
		/// The maximum size of the encoded app specific metadata
		#[pallet::constant]
		type MaxEncodedAppMetadata: Get<u32>;
		/// The maximum number of owners per capsule/document
		#[pallet::constant]
		type MaxOwners: Get<u32>;
		/// The maximum length of a capsule key in a container stored on-chain.
		#[pallet::constant]
		type StringLimit: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn capsules)]
	pub type Capsules<T> = StorageMap<_, Twox64Concat, CapsuleIdFor<T>, CapsuleMetadataOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn followers)]
	pub type CapsuleFollowers<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Twox64Concat,
		CapsuleIdFor<T>,
		Follower,
	>;

	#[pallet::storage]
	#[pallet::getter(fn document_get)]
	pub type Document<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		DocumentIdOf<T>,
		Blake2_128Concat,
		KeyOf<T>,
		CapsuleIdFor<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn document_details)]
	pub type DocumentDetails<T: Config> =
		StorageMap<_, Twox64Concat, DocumentIdOf<T>, DocumentDetailsOf<T>>;

	/// Events that functions in this pallet can emit.
	///
	/// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
	/// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
	/// documentation for each event field and its parameters is added to a node's metadata so it
	/// can be used by external interfaces or tools.
	///
	///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
	/// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
	/// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]

	pub enum Event<T: Config> {
		/// A user has successfully set a new value.
		SomethingStored {
			/// The new value set.
			something: u32,
			/// The account who set the new value.
			who: T::AccountId,
		},
	}

	/// Errors that can be returned by this pallet.
	///
	/// Errors tell users that something went wrong so it's important that their naming is
	/// informative. Similar to events, error documentation is added to a node's metadata so it's
	/// equally important that they have helpful documentation associated with them.
	///
	/// This type of runtime error can be up to 4 bytes in size should you want to return additional
	/// information.
	#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Create tokens dispatchable function
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(100_000, 0))]
		pub fn create_tokens(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let _who = ensure_signed(origin)?;

			// Return a successful `DispatchResult`
			Ok(())
		}
	}
}
