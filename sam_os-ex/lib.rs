#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;

#[ink::contract]
mod sam_contract {
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        traits::{SpreadAllocate, SpreadLayout},
        Mapping,
    };

    type DID = Vec<u8>;
    type IpfsCid = Vec<u8>;
    type HashKey = Vec<u8>;
    type AuthContent = Vec<u8>;
    type DatabaseMetadata = Vec<u8>;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    struct FileMeta {
        access_list: Vec<AccountId>,
        cid: IpfsCid,
        timestamp: u64,
        db_meta: DatabaseMetadata,
    }

    // #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    // // #[derive(SpreadAllocate, SpreadLayout)]
    // #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    // struct UserInfo {
    //     auth_content: AuthContent,
    //     did_doc_cid: IpfsCid,
    // }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct SamOs {
        /// Storage for DIDs and their documents and auth material
        auth_list: Mapping<DID, (Vec<u8>, Vec<u8>)>,
        /// Storage for user documents metadata
        files_meta: Mapping<HashKey, FileMeta>,
        /// Stores the access list of a file for easy retreival
        access_list: Mapping<DID, (HashKey, u64)>,
    }

    impl SamOs {
        /// Constructor that initializes all the contract storage to default
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        #[ink(message)]
        pub fn create_new_account(
            &mut self,
            did: DID,
            auth_content: AuthContent,
            did_doc_cid: IpfsCid,
        ) -> Result<(), String> {
            let user = (auth_content, did_doc_cid);

            self.auth_list.insert(did, &user);

            Ok(())
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     // use super::*;

    //     #[ink::test]
    //     fn it_works() {
    //         // let mut sam = SamOs::new();
    //         // assert_eq!(sam.create_new_account(
    //         //     "did:sam:root:DSOSAI0M83YAULZZLR6DEJMD3IWDKDHCRMJAR4HTFBHCW".to_string(),
    //         //     "DSOSAI0M83YAULZZLR6DEJMD3IWDKDHCRMJA".to_string(),
    //         //     "uhskjkfuai90arf0j9afsda".to_string(),
    //         // ), Ok(()));
    //     }

    //     #[ink::test]
    //     fn check() {
    //         let hk = "wonder";
    //         let length = hk.len() - 2;
    //         let sub_hk = &hk[0..length]; 

    //         assert_eq!(sub_hk, "wond");
    //     }
    // }
}
