use std::error::Error;
use workspaces::{Account, Contract};

pub struct Space {
    pub account: Account,
    pub contract: Contract
}

impl Space {
    pub fn new(account: Account, contract: Contract) -> Self {
        Self { account, contract }
    }
}

pub fn print_passed(name: &str) {
    println!("Passed ✅ {name}");
}

pub fn print_failed(name: &str, err: Box<dyn Error>) {
    println!("Failed ❌ {name}: {err}");
}