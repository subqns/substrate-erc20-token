#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Erc20 {
        /// The total supply.
        total_supply: Balance,
        /// The balance of each user.
        balances: ink_storage::collections::HashMap::<AccountId, Balance>,
        /// allowance map
        allowance: ink_storage::collections::HashMap::<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        amount: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        ForbiddenTransfer,
    }
    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = ink_storage::collections::HashMap::<AccountId, Balance>::new();
            let mut allowance = ink_storage::collections::HashMap::<(AccountId, AccountId), Balance>::new();
            let caller = Self::env().caller();
            balances.insert(caller, total_supply);
            Self {
                total_supply: total_supply,
                balances: balances,
                allowance: allowance,
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner : AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn allowance_of(&self, owner : AccountId, spender : AccountId) -> Balance {
            *self.allowance.get(&(owner, spender)).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn approve(&mut self, owner: AccountId, spender: AccountId, amount: Balance) -> Result<()> {
            let owner_balance = self.balance_of(owner);
            if amount > owner_balance {
                return Err(Error::InsufficientBalance)
            }

            self.allowance.insert((owner, spender), amount);

            self.env().emit_event(Approval{ owner, spender, amount });

            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = Self::env().caller();
            self._transfer_helper(from, to, value)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<()> {
            if amount > self.allowance_of(from, to) {
                return Err(Error::ForbiddenTransfer)
            }
            self._transfer_helper(from, to, amount)
        }

        fn _transfer_helper(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<()> {
            let (from_balance, to_balance) = (self.balance_of(from), self.balance_of(to));
            if from_balance < amount {
                return Err(Error::InsufficientBalance)
            }
            self.balances.insert(from, from_balance - amount);
            self.balances.insert(to, to_balance + amount);

            self.env().emit_event(Transfer{ from, to, amount });

            Ok(())
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
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
        }
    }
}
