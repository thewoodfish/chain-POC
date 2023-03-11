#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod batman {
    use scale_info::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Batman {
        assistant: Vec<u8>,
    }

    impl Batman {
        #[ink(constructor)]
        pub fn new(assistant: Vec<u8>) -> Self {
            Self { assistant }
        }

        #[ink(message)]
        pub fn get_assistant(&self) -> Vec<u8> {
            self.assistant.clone()
        }

        #[ink(message)]
        pub fn set_assistant(&mut self, name: Vec<u8>) {
            self.assistant = name;
        }

    }

    // #[cfg(test)]
    // mod tests {  001400670072006500610074
        // cargo contract instantiate --constructor new --args  \[20, 103, 114, 101, 97, 116\] --suri //Alice --salt $(date +%s)
        // cargo contract call --contract 5E3rCrW49guH5V9HMrBqys76nSgkzneaLvRvupZzT7DEoqj9 --message set_assistant --args 0x146772656174 --suri //Alice --dry-run
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let batman = Batman::default();
    //         assert_eq!(batman.get(), false);
    //     }

    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut batman = Batman::new(false);
    //         assert_eq!(batman.get(), false);
    //         batman.flip();
    //         assert_eq!(batman.get(), true);
    //     }
    // }
}
