use anyhow::{bail, Error as AnyError};
use coinflip::flip;
use thiserror::Error as ThisError;

pub fn can_fail_1() -> Result<(), AnyError> {
    if flip() {
        bail!("Hmm something went wrong ")
    }
    Ok(())
}

#[derive(ThisError, Debug)]
pub enum MyError2 {
    #[error("unknown error occured")]
    Unknown,
}

pub fn can_fail_2() -> Result<(), MyError2> {
    if flip() {
        return Err(MyError2::Unknown);
    }
    Ok(())
}

pub fn main() {
    match can_fail_1() {
        Ok(_) => {
            //
        }
        Err(why) => println!("fn 'can_fail_1' failed because: {}", why),
    }

    match can_fail_2() {
        Ok(_) => {
            //
        }
        Err(why) => println!("fn 'can_fail_2' failed because: {}", why),
    }
}
