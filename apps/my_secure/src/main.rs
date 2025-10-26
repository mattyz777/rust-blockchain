mod aes256gcm_util;
mod result_err;
mod hex_util;

use aes256gcm_util::test_Aes256GcmUtil;
use anyhow::Result;
use result_err::caller;

fn main() -> Result<()> {
    // test_Aes256GcmUtil()?;
    // caller()?;
    hex_util::caller()?;
    Ok(())
}