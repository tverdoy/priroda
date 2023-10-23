mod utils;
mod user;

use std::{env, fs};
use crate::utils::Space;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;
    let space = Space::run(wasm_filepath).await?;

    println!("Run integration tests");

    user::test::run_all_tests(&space).await?;


    println!("All tests is passed");
    Ok(())
}


