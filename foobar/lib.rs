#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod foobar {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    
    pub struct MyStruct {
        /// Stores a single `bool` value on the storage.
        value: bool,

        my_string: String,

        my_vector: Vec<u32>,

        my_account: AccountId,

        my_balance: Balance,
        
        my_hash: Hash,
    }

    pub enum Status {
        NotSTarted,
        OpeningPeriod,
    }

    pub struct Auction {
        name: String,
        subject: Hash,
        status: Status,
        finalized: bool,
        vector: Vec<u8>,
    }

    #[ink(event)]
    pub struct Created {
        #[ink(topic)]   // Indexed element
        message: String,
    }

    #[ink(event)]
    pub struct Flipped {
        #[ink(topic)]   // Indexed element
        flip: bool,

    }

    #[ink(storage)]
    pub struct Foobar {
        value: bool,
    }
    
    impl Foobar {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self::env().emit_event(Created {
                message: String::from("Foobar created")
            });
            Self {
                value: init_value,
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        /// 
        /// Selector calculation
        /// 
        /// 1. Grab the name of the message or constructor
        ///  
        /// 2. Compute the BLACKE2 hash of the name
        /// 
        /// 3. Take the first 4 bytes of the hash as the selector
        /// 
        /// BLACKE2(flip) = 0x633aa551........................
        /// 
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;

            self.env().emit_event(Flipped {
                flip: self.value
            });
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let foobar = Foobar::default();
            assert_eq!(foobar.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut foobar = Foobar::new(false);
            assert_eq!(foobar.get(), false);
            foobar.flip();
            assert_eq!(foobar.get(), true);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = FoobarRef::default();

            // When
            let contract_account_id = client
                .instantiate("foobar", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<FoobarRef>(contract_account_id.clone())
                .call(|foobar| foobar.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = FoobarRef::new(false);
            let contract_account_id = client
                .instantiate("foobar", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<FoobarRef>(contract_account_id.clone())
                .call(|foobar| foobar.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<FoobarRef>(contract_account_id.clone())
                .call(|foobar| foobar.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<FoobarRef>(contract_account_id.clone())
                .call(|foobar| foobar.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
