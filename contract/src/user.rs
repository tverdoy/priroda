use crate::{Contract, ContractExt};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId};


/// User is main unit in social network.
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct User {
    pub id: AccountId,
    pub nickname: String,
}

impl User {
    pub fn new(id: AccountId, nickname: String) -> Self {
        Self { id, nickname }
    }
}

#[near_bindgen]
impl Contract {
    pub fn register_user(&mut self, nickname: String) -> User {
        if nickname.len() > 255 {
            env::panic_str("nickname length cannot be greater than 255")
        }

        let user = User::new(env::predecessor_account_id(), nickname);
        self.users.insert(&user.id, &user);
        user
    }

    pub fn get_user(&self, account_id: AccountId) -> User {
        match self.users.get(&account_id) {
            Some(user) => user,
            None => env::panic_str("user not found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        for _ in 0..256 {
            nickname += "n"
        }

        contract.register_user(nickname);
    }
}
