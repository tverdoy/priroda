use serde_json::json;
use workspaces::{AccountId};
use serde::Deserialize;
use crate::utils::Space;

const NICKNAME: &str = "a1ce";


/// User in contract. More see in contract
#[derive(Debug, Deserialize, PartialEq)]
pub struct User {
    pub id: AccountId,
    pub nickname: String
}

impl User {
    /// Run register user action.
    ///
    /// Register main account (alice)
    pub async fn register(
        space: &Space
    ) -> anyhow::Result<User> {
        space.alice
            .call( space.contract.id(), "register_user")
            .args_json(json!({"nickname": NICKNAME}))
            .transact()
            .await?
            .json().map_err(|e| e.into())
    }

    /// View get user
    ///
    /// Get by main account id (alice)
    pub async fn get(
        space: &Space
    ) -> anyhow::Result<User> {
        space.alice
            .call( space.contract.id(), "get_user")
            .args_json(json!({"account_id": space.alice.id()}))
            .transact()
            .await?
            .json().map_err(|e| e.into())
    }
}

pub(crate) mod test {
    use crate::utils::{print_failed, print_passed};
    use super::*;

    pub(crate) async fn run_all_tests(space: &Space) -> anyhow::Result<()> {
        let tests = [
            (register_user, "register user")
        ];

        for test in tests {
            match test.0(space).await {
                Ok(_) => print_passed(test.1),
                Err(e) => print_failed(test.1, e.into())
            }
        }

        Ok(())
    }

    async fn register_user(space: &Space)-> anyhow::Result<()> {
        let user = User::register(space).await?;
        assert_eq!(user.nickname, NICKNAME);

        let user_getter = User::get(space).await?;
        assert_eq!(user, user_getter);

        Ok(())
    }
}