#![cfg_attr(feature = "no_std", no_std)]
#![cfg_attr(feature = "no_std", no_main)]
mod no_std;

use anyhow::Result;

#[cfg(not(feature = "no_std"))]
fn main() -> Result<()> {
    Ok(())
}
