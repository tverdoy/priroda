use serde_json::json;
use workspaces::{Account, AccountId};
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
    pub async fn register(
        space: &Space,
        account: &Account,
        nickname: String
    ) -> workspaces::Result<User> {
        account
            .call( space.contract.id(), "register_user")
            .args_json(json!({"nickname": nickname}))
            .transact()
            .await?
            .json().map_err(|e| e.into())
    }

    /// View get user
    pub async fn get(
        space: &Space,
        account_id: &AccountId
    ) -> workspaces::Result<User> {
        space.alice
            .call( space.contract.id(), "get_user")
            .args_json(json!({"account_id": account_id}))
            .transact()
            .await?
            .json()
    }

    /// View get user by main account (alice)
    pub async fn get_main(space: &Space) -> workspaces::Result<User> {
        Self::get(space, space.alice.id()).await
    }
}

pub(crate) mod test {
    use anyhow::Error;
    use crate::utils::{print_failed, print_passed};
    use super::*;

    pub(crate) async fn run_all_tests(space: &Space) -> anyhow::Result<()> {
        let name = "register user";
        match register_user(space).await {
            Ok(_) => print_passed(name),
            Err(e) => print_failed(name, e.into())
        }

        let name = "get not exists user";
        match get_not_exists_user(space).await {
            Ok(_) => print_passed(name),
            Err(e) => print_failed(name, e.into())
        }

        Ok(())
    }

    async fn register_user(space: &Space)-> anyhow::Result<()> {
        let user = User::register(space, &space.alice, NICKNAME.to_string()).await?;
        assert_eq!(user.nickname, NICKNAME);

        let user_getter = User::get_main(space).await?;
        assert_eq!(user, user_getter);

        Ok(())
    }

    async fn get_not_exists_user(space: &Space)-> anyhow::Result<()> {
        let res = User::get(space, &"notexistsaccount.near".parse().unwrap()).await;
        match res {
            Ok(_) => return Err(Error::msg("user get must be return error")),
            Err(e) => assert_eq!(e.kind(), &workspaces::error::ErrorKind::Execution)
        }

        Ok(())
    }
}