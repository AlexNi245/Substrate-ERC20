#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_storage::collections::HashMap;

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: HashMap<AccountId, Balance>,
        approvals: HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        from: AccountId,
        to: AccountId,
        amount: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut _balances: HashMap<AccountId, Balance> = HashMap::new();
            _balances.insert(Self::env().caller(), total_supply);

            Self {
                total_supply,
                balances: _balances,
                approvals: HashMap::new(),
            }
        }
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            return self.total_supply;
        }
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> bool {
            let sender = self.env().caller();
            return self._transfer(sender, to, amount);
        }
        #[ink(message)]
        pub fn balance_of(&self, account_id: AccountId) -> Balance {
            return *self.balances.get(&account_id).unwrap_or(&0);
        }

        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, amount: Balance) {
            let sender = self.env().caller();
            self.approvals.insert((sender, to), amount);
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: Balance) {
            let caller = self.env().caller();
            let allowed = self.approvals.get(&(from, caller)).unwrap_or(&0).clone();

            if &amount > &allowed {
                return;
            };

            let transfer_result = self._transfer(from, to, amount);

            if transfer_result == false {
                return;
            }

            self.approvals.insert((from, to), allowed - amount);
        }
        fn _transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> bool {
            let senders_balance = *self.balances.get(&from).unwrap_or(&0);

            if senders_balance < amount {
                return false;
            }
            self.balances.insert(from, senders_balance - amount);

            let receivers_balance = *self.balances.get(&to).unwrap_or(&0);
            self.balances.insert(to, receivers_balance + amount);

            self.env().emit_event(Transfer {
                from: from,
                to,
                amount,
            });
            return true;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let contract = Erc20::new(777);
            assert_eq!(contract.total_supply(), 777);
        }

        #[ink::test]
        fn balance_works() {
            let contract = Erc20::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert!(contract.transfer(AccountId::from([0x0; 32]), 10));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
            assert!(!contract.transfer(AccountId::from([0x0; 32]), 100));
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            contract.approve(AccountId::from([0x1; 32]), 20);
            contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 10);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
        }
    
    }
}
