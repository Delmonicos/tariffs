#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod fixed_price {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct FixedPrice {
        /// Stores a single `bool` value on the storage.
        price: u8,
    }

    impl FixedPrice {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: u8) -> Self {
            Self { price: init_value }
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
        #[ink(message)]
        pub fn update_price(&mut self, new_price: u8) {
            self.price = new_price;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_price(&self) -> u8 {
            self.price
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
        #[test]
        fn default_works() {
            let fixed_price = FixedPrice::default();
            assert_eq!(fixed_price.get_price(), 0);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut fixed_price = FixedPrice::new(0);
            assert_eq!(fixed_price.get_price(), 0);
            fixed_price.update_price(5);
            assert_eq!(fixed_price.get_price(), 5);
        }
    }
}
