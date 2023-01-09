pub mod error {
    use anyhow::{bail, Error as AnyError};
    use coinflip::flip;
    use thiserror::Error as ThisError;

    // returns anyerror
    pub fn can_fail_1() -> Result<(), AnyError> {
        if flip() {
            bail!("Hmm something went wrong ")
        }
        Ok(())
    }

    #[derive(ThisError, Debug)]
    pub enum MyError {
        #[error(transparent)]
        Anyhow(#[from] anyhow::Error),
        #[error("unknown error occured")]
        Unknown,
    }

    // returns myerror
    pub fn can_fail_2() -> Result<(), MyError> {
        if flip() {
            return Err(MyError::Unknown);
        }
        Ok(())
    }

    pub fn can_fail() -> Result<(), MyError> {
        match can_fail_1() {
            Ok(_) => match can_fail_2() {
                Ok(_) => Ok(()),
                Err(why) => {
                    println!("fn 'can_fail_2' failed because: {}", why);
                    Err(why)
                }
            },
            Err(why) => {
                println!("fn 'can_fail_1' failed because: {}", why);
                // converts anyhow to myerror
                Err(why.into())
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_it() {
            can_fail().ok();
        }
    }
}

#[allow(dead_code)]
mod type_level_program {
    use std::marker::PhantomData;

    // Each state is a unique type
    struct Mutable;
    struct Immutable;

    // #[repr(transparent)]
    struct MyStateMachine<State> {
        data: u32,
        _phantom: PhantomData<State>,
    }

    impl MyStateMachine<Mutable> {
        pub fn new() -> Self {
            Self {
                data: 0,
                _phantom: PhantomData::<Mutable>,
            }
        }

        pub fn increment(&mut self) {
            self.data += 1;
        }

        pub fn get_data(&self) -> u32 {
            self.data
        }

        pub fn lock(&mut self) -> MyStateMachine<Immutable> {
            MyStateMachine {
                data: self.data,
                _phantom: PhantomData::<Immutable>,
            }
        }
    }

    impl MyStateMachine<Immutable> {
        pub fn new() -> Self {
            Self {
                data: 0,
                _phantom: PhantomData::<Immutable>,
            }
        }

        pub fn increment(&mut self) {
            println!("Can't increment in a locked state!");
        }

        pub fn get_data(&self) -> u32 {
            self.data
        }

        pub fn unlock(&mut self) -> MyStateMachine<Mutable> {
            MyStateMachine {
                data: self.data,
                _phantom: PhantomData::<Mutable>,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        pub fn state_machine_transitions() {
            let mut instance = MyStateMachine::<Mutable>::new();
            instance.increment();

            assert_eq!(instance.data, 1u32);

            let mut instance = instance.lock();

            instance.increment();
            assert_eq!(instance.data, 1u32);

            let mut instance = instance.unlock();
            assert_eq!(instance.data, 1u32);

            instance.increment();
            assert_eq!(instance.data, 2u32);
        }
    }
}

// cargo test --features malicious
mod malicious_feature {

    #[allow(dead_code)]
    fn evilify(data: &str) -> &str {
        #[cfg(feature = "malicious")]
        return "evil";
        data
    }

    #[cfg(test)]
    mod tests {
        use super::evilify;

        #[test]
        fn for_malicious() {
            let out = evilify("good");

            if cfg!(feature = "malicious") {
                assert_eq!(out, "evil");
            } else {
                assert_eq!(out, "good");
            }
        }
    }
}

// TODO
/*
implement a program that loops 1000 times, repeatedly branching on secret data (say, equality to number 123456789012345678), taking the left path in execution A and the right path in execution B.
Benchmark your program.
Determine if your benchmarks are statistically different.
Try this first with a single u64, then repeat the experiment making your secret data a vector of length 100 u64's.
 */
pub mod secret_data_branching {

    pub const SECRET: u64 = 123456789012345678;

    pub fn fn_with_side_channel(value: u64) -> (u32, u32) {
        let mut left = 0;
        let mut right = 0;

        for _i in 0..1000 {
            match value {
                SECRET => left += 1,
                _ => right += 1,
            }
        }

        (left, right)
    }
}
