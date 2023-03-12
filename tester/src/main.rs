/// this crate is to test SamaritanOS contract and parse its return data
/// in preparation for samaitanDB integration
use std::process::Command;

fn main() {
    // test functions
    // test_account_creation();
    // test_is_auth();
    // test_file_addition();
    // test_getting_sync_info();\

    let real_info = test_get_file_info();
    let extra_info = test_get_extra_info();


    println!("{}", real_info);
    println!("{}", extra_info);

    // // println!("success? {}", output.status.success());
    // println!("success? {:#?}", output);
    // let binding = String::from_utf8_lossy(&output.stdout);
    // // if parse_contract_boolean(&binding) {
    // //     println!("The account exists!");
    // // } else {
    // //     println!("The account doesn't exists!")
    // // }
    // println!("{}", binding);
    // println!("u64 -> {}", parse_first_tuple_u64(&binding));
    // println!("Result -> {}", parse_contract_return_data(&binding));
    // let num: u64 = 1002;
    // let io: Vec<u8> = num.to_ne_bytes().to_vec();
    // println!("{:?}", String::from_utf8(io).unwrap());
}

// cargo contract call --contract 5E3rCrW49guH5V9HMrBqys76nSgkzneaLvRvupZzT7DEoqj9 --message get_assistant --suri //Alice --dry-run
// 5GeKCcowiWzVdgnBgchCdreHUbsLSSCmndrxsPdwrTJGK1nH

fn test_get_file_info() -> String {
    let did = "did:sam:root:jhjbsfgiusif78uwn9s980h9";
    let output = Command::new("cargo")
        .args([
            "contract",
            "call",
            "--contract",
            "5GeKCcowiWzVdgnBgchCdreHUbsLSSCmndrxsPdwrTJGK1nH",
            "--message",
            "get_files_info",
            "--args",
            &str_to_hex(did),
            "--suri",
            "//Alice",
            "--dry-run",
        ])
        .current_dir("../sam_os")
        .output()
        .expect("failed to execute process");

    let binding = String::from_utf8_lossy(&output.stdout);
    parse_contract_return_data(&binding)
}

fn test_get_extra_info() -> String {
    let did = "did:sam:root:jhjbsfgiusif78uwn9s980h9";
    let output = Command::new("cargo")
        .args([
            "contract",
            "call",
            "--contract",
            "5GeKCcowiWzVdgnBgchCdreHUbsLSSCmndrxsPdwrTJGK1nH",
            "--message",
            "get_files_extra_info",
            "--args",
            &str_to_hex(did),
            "--suri",
            "//Alice",
            "--dry-run",
        ])
        .current_dir("../sam_os")
        .output()
        .expect("failed to execute process");

    let binding = String::from_utf8_lossy(&output.stdout);
    parse_contract_tuple_vector(&binding)
}

fn test_file_addition() {
    let cid = "QmsiunSlonNdhuIjNHuUtfkhjOijoho";
    let hashkey: u64 = 3656788;
    let metadata = "Algorealms SamaritanDB";
    let dids = [
        "did:sam:root:jhjbsfgiusif78uwn9s980h9".to_string(),
        "did:sam:root:apps:xxxxxxxxxxxx".to_string(),
    ];
    let access_bits = [true, true];
    let output = Command::new("cargo")
        .args([
            "contract",
            "call",
            "--contract",
            "5GeKCcowiWzVdgnBgchCdreHUbsLSSCmndrxsPdwrTJGK1nH",
            "--message",
            "update_file_meta",
            "--args",
            &str_to_hex(cid),
            &str_to_hex(&format!("{}", hashkey)),
            &str_to_hex(metadata),
            &str_to_hex(&dids[0]),
            &str_to_hex(&dids[1]),
            &format!("{}", access_bits[0]),
            &format!("{}", access_bits[1]),
            "--suri",
            "//Alice",
        ])
        .current_dir("../sam_os")
        .output()
        .expect("failed to execute process");
}

fn test_getting_sync_info() {
    let hashkey: u64 = 39039222;

    let output = Command::new("cargo")
        .args([
            "contract",
            "call",
            "--contract",
            "5GeKCcowiWzVdgnBgchCdreHUbsLSSCmndrxsPdwrTJGK1nH",
            "--message",
            "get_file_sync_info",
            "--args",
            &str_to_hex(&format!("{}", hashkey)),
            "--suri",
            "//Alice",
            "--dry-run",
        ])
        .current_dir("../sam_os")
        .output()
        .expect("failed to execute process");

    println!("success? {}", output.status.success());
    println!("success? {:#?}", output);
    // let binding = String::from_utf8_lossy(&output.stdout);
    // if parse_contract_boolean(&binding) {
    //     println!("The account exists!");
    // } else {
    //     println!("The account doesn't exists!")
    // }

    // println!("{}", binding);
    // println!("u64 -> {}", parse_first_tuple_u64(&binding));
    // println!("CID -> {}", parse_contract_return_data(&binding));
}

fn test_is_auth() {
    let output = Command::new("cargo")
        .args([
            "contract",
            "call",
            "--contract",
            "5GeKCcowiWzVdgnBgchCdreHUbsLSSCmndrxsPdwrTJGK1nH",
            "--message",
            "account_is_auth",
            "--args",
            &str_to_hex("did:sam:root:jhjbsfgiusif78uwn9s980h9"),
            &str_to_hex(&format!("{}", 32323232)),
            "true",
            "--suri",
            "//Alice",
            "--dry-run",
        ])
        .current_dir("../sam_os")
        .output()
        .expect("failed to execute process");

    let binding = String::from_utf8_lossy(&output.stdout);
    if parse_contract_boolean(&binding) {
        println!("The account exists!");
    } else {
        println!("The account doesn't exists!")
    }
}

fn test_account_creation() {
    let output = Command::new("cargo")
        .args([
            "contract",
            "call",
            "--contract",
            "5GeKCcowiWzVdgnBgchCdreHUbsLSSCmndrxsPdwrTJGK1nH",
            "--message",
            "create_new_account",
            "--args",
            &str_to_hex("did:sam:root:jhjbsfgiusif78uwn9s980h9"),
            &str_to_hex(&format!("{}", 32323232)),
            &str_to_hex("empty"),
            "--suri",
            "//Alice",
        ])
        .current_dir("../sam_os")
        .output()
        .expect("failed to execute process");

    println!("success? {}", output.status.success())
}

/// parse contract boolean data
pub fn parse_contract_boolean(data: &str) -> bool {
    let io = data.split("Bool(").skip(1).next().unwrap_or_default();
    io.chars().nth(0).unwrap_or('f') == 't'
}

/// Parses the first integreg(u64) in a tuple
pub fn parse_first_tuple_u64(binding: &str) -> u64 {
    let output = binding.split("elems: ").next().unwrap_or_default();
    // lets the number out
    let parsed = output
        .as_bytes()
        .to_vec()
        .iter()
        .filter(|&&e| e.is_ascii_digit())
        .map(|e| e.clone())
        .collect::<Vec<u8>>();

    let str = String::from_utf8(parsed).unwrap_or_default();
    str.parse::<u64>().unwrap_or_default()
}

// parses contract return data and returns it as a human readable string
pub fn parse_contract_return_data(binding: &str) -> String {
    let output = binding.split("elems: ").skip(1).next().unwrap_or_default();
    let mut collator: Vec<u8> = Vec::new();
    // lets get all the numbers out
    let parsed = output
        .as_bytes()
        .to_vec()
        .iter()
        .filter(|&&e| e == b',' || e.is_ascii_digit())
        .map(|e| e.clone())
        .collect::<Vec<u8>>();

    let _ = String::from_utf8(parsed)
        .unwrap_or_default()
        .split(',')
        .map(|e| {
            collator.push(e.parse::<u8>().unwrap_or_default());
        })
        .collect::<()>();

    String::from_utf8(collator).unwrap_or_default()
}

// accepts a string and returns its hexadecimal format
pub fn str_to_hex(data: &str) -> String {
    format!("{}{}", "0x", hex::encode(data))
}

pub fn parse_contract_tuple_vector(binding: &str) -> String {
    let parsed = binding
        .as_bytes()
        .to_vec()
        .iter()
        .filter(|&&e| e == b',' || e.is_ascii_digit())
        .map(|e| e.clone())
        .collect::<Vec<u8>>();

    String::from_utf8(parsed).unwrap_or_default()
}

// #[ink(message)]
//         pub fn create_new_account(
//             &mut self,
//             did: DID,
//             auth_content: AuthContent,
//             did_doc_cid: IpfsCid,
//         ) -> Result<()> {
//             let user = UserInfo {
//                 auth_content,
//                 did_doc_cid,
//             };

//             self.auth_list.insert(did, &user);
//             Ok(())
//         }

// 5EK4Wuz19DFPSH4bH51r4PZJXwxidE24xgXkUYHQTB9MQQ24  ---->   5HRbmEmnNh25Qb3BZggsVGUCmzDLSfY74TZRsCRzggKbbfiz
