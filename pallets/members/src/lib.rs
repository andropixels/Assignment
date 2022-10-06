#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn members)]
    
	pub type Members<T:Config> = StorageMap<_,Blake2_128Concat, u32,T::AccountId,OptionQuery>;
    
    #[pallet::storage]
	#[pallet::getter(fn root)]
    ///strage to store the root
	
	
	pub type Root<T:Config> = StorageValue<_, T::AccountId>;
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		///MemberAdded
		MemberAdded( T::AccountId),
        	///MemberRemoved
        	MemberRemoved(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// origin is not a root.
		NotARoot,
        ///root already declared and cannot be stored again 
        RootAlreadySet
		
		
	}
    
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
	
        
     
        #[pallet::weight(10_000)]
        pub fn set_root(origin:OriginFor<T>) -> DispatchResult {
                  
            let who = ensure_signed(origin)?;
           // Read a value from storage.
		// Not checking while updating storage
			match <Root<T>>::get() {
				// Return an error if the value has not been set.
				None =>{
                    <Root<T>>::put(who);
                    
                    Ok(())
                },
				Some(old) => {
				Err(Error::<T>::RootAlreadySet)?
				},
			}
         

             
        }

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_member(origin: OriginFor<T>, member: T::AccountId) -> DispatchResult {
			
			let who = ensure_signed(origin)?;
            let root =<Root<T>>::get();
            let auth = root.unwrap();
            if who != auth {
              Err(Error::<T>::NotARoot)?  
            }

			// Update storage.
            let mut member_count =0; 
			<Members<T>>::insert(member_count,&member);
            member_count +=1 ;


			// Emit an event.
			Self::deposit_event(Event::MemberAdded( member));
			
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn remove_member(origin: OriginFor<T>,member_id:u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
            let root =<Root<T>>::get();
            let auth = root.unwrap();
            if who != auth {
              Err(Error::<T>::NotARoot)?  
            }
			
			let member = <Members<T>>::take(member_id);
            <Members<T>>::remove(member_id);


            Self::deposit_event(Event::MemberRemoved( member.unwrap()));
            Ok(())
		}
	}
}
