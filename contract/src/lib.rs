pub mod user;

use crate::user::User;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    users: LookupMap<AccountId, User>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            users: LookupMap::new(b"u"),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::env;

    #[test]
    fn register_user() {
        let mut contract = Contract::default();
        let nickname = "stray";
        contract.register_user(nickname.to_string());
        let user = contract.get_user(env::predecessor_account_id());
        assert_eq!(nickname, user.nickname)
    }

    #[test]
    #[should_panic]
    fn max_length_nickname() {
        let mut contract = Contract::default();
        let mut nickname = String::new();

        for i in 0..256 {
            nickname += "n"
        }

        contract.register_user(nickname);
    }
}
