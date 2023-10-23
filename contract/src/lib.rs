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


