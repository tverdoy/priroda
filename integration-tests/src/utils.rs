use std::error::Error;
use std::path::{PathBuf};
use near_units::parse_near;
use workspaces::{Account, Contract};

/// Wrap for working with near network
///
/// Main account is alice
pub struct Space {
    pub alice: Account,
    pub bob: Account,
    pub contract: Contract
}

impl Space {
    /// Run workspace and deploy contract
    ///
    /// Also create 2 users, alice and bob
    pub async fn run(target: PathBuf) -> anyhow::Result<Self> {
        let worker = workspaces::sandbox().await?;
        let wasm = std::fs::read(target)?;
        let contract = worker.dev_deploy(&wasm).await?;

        // create accounts
        let account = worker.dev_create_account().await?;
        let alice = account
            .create_subaccount( "alice")
            .initial_balance(parse_near!("30 N"))
            .transact()
            .await?
            .into_result()?;

        let bob = account
            .create_subaccount( "bob")
            .initial_balance(parse_near!("30 N"))
            .transact()
            .await?
            .into_result()?;

        Ok(Self { alice, bob, contract })
    }
}

/// Print format message about test is passed
pub fn print_passed(name: &str) {
    println!("Passed ✅ {name}");
}

/// Print format massage about test is failed
pub fn print_failed(name: &str, err: Box<dyn Error>) {
    println!("Failed ❌ {name}: {err}");
}