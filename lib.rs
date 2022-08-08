#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::new_without_default)]

use ink_lang as ink;

#[ink::contract]
mod contract_transfer {

    // use ink_env::AccountId;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ContractTransfer {
        // Store a mapping from AccountIds to Balance
        map: ink_storage::Mapping<AccountId, Balance>,
    }

    impl ContractTransfer {
        /// Constructor that initializes the value of map: Account Id and balance.
        #[ink(constructor)]
        pub fn new(init_balance: Balance) -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.map.insert(&caller, &init_balance);
            })
        }

        /// Function default for initializing storage
        #[ink(constructor)]
        pub fn default() -> Self {
            // default value for contract
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// Set balance
        #[ink(message)]
        pub fn add_balance(&mut self, balance: Balance) {
            let caller = Self::env().caller();
            let current_balance = self.map.get(&caller).unwrap_or_default();
            let total_balance = current_balance + balance;
            self.map.insert(&caller, &total_balance); 
        }

        /// Transfer balance from AccountId to AccountId
        #[ink(message)]
        pub fn transfer_balance(&mut self, value: Balance, to: AccountId) {
            ink_env::debug_println!("requested value: {}", value);

            let from = Self::env().caller();
            let mut from_balance = self.map.get(&from).unwrap_or_default();
            let mut to_balance = self.map.get(&to).unwrap_or_default();

            // Check insufficient balance
            assert!(value <= from_balance, "insufficient funds!");

            // Balance of account after transfer
            from_balance -= value;
            to_balance += value;

            // Add to storage
            self.map.insert(from, &from_balance); 
            self.map.insert(to, &to_balance); 
        }

        /// Grab the balance at the caller's AccountID, if it exists
        #[ink(message)]
        pub fn get(&self) -> Balance {
            let caller = Self::env().caller();
            self.map.get(&caller).unwrap_or_default()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let contract_transfer = ContractTransfer::default();
            assert_eq!(contract_transfer.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works_add_balance() {
            let mut contract_transfer = ContractTransfer::new(0);
            assert_eq!(contract_transfer.get(), 0);
            contract_transfer.add_balance(100);
            assert_eq!(contract_transfer.get(), 100);
        }

        /// We test when transfer balance
        #[ink::test]
        fn it_works_transfer_balance() {
            // Create account "to", contractTransfer, init balance = 100
            let accounts = default_accounts();
            let mut contract_transfer = ContractTransfer::new(100);
            let to_account = accounts.eve;

            // Implement transfer balance
            contract_transfer.transfer_balance(80, to_account);

            // Check balance "to account" when completed transfer
            assert_eq!(contract_transfer.get(), 20);

        }

        /// We test when insufficient balance
        #[ink::test]
        #[should_panic(expected = "insufficient funds!")]
        fn transfer_fails_insufficient_funds() {
            // Create account "to", contractTransfer, init balance = 100
            let accounts = default_accounts();
            let mut contract_transfer = ContractTransfer::new(100);
            let to_account = accounts.eve;

            // Implement transfer balance
            contract_transfer.transfer_balance(150, to_account);

            // then
            // `transfer_balance` must already have panicked here
        }

        // Function get default_accounts
        fn default_accounts(
        ) -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
        }
    }
}
