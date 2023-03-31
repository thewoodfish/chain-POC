/// this crate is to test SamaritanOS contract and parse its return data
/// in preparation for samaitanDB integration
use std::process::Command;

fn main() {
    // test functions
    // test_account_creation();
    // test_hashtable_update();

    // test_is_auth();
    pull_from_ipfs("QmR6bcs7yv7qDK73mvAwXAFqVP2cRi31z4hBddhVuKSrko");
    // test_file_addition();
    // test_getting_sync_info();\

    // let real_info = test_get_file_info();
    // let extra_info = test_get_extra_info();

    // println!("{}", real_info);
    // println!("{}", extra_info);

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
    // println!(
    //     "{:#?}, {:#?}, {:#?}",
    //     &str_to_hex("did:sam:root:jhjbsfgiusif78uwn9s980h9"),
    //     &str_to_hex(&format!("{}", 32323232)),
    //     &str_to_hex("empty"),
    // );
}

// "contract",
//             "call",
//             "--contract",
//             "5EHqUsDvHKTDMT6qsGbagfgGgPRy8YSqtCKkGdexfpt84WdU",
//             "--message",
//             "create_new_account",
//             "--args",
//             &str_to_hex("did:sam:root:jhjbsfgiusif78uwn9s980h9"),
//             &str_to_hex(&format!("{}", 32323232)),
//             &str_to_hex("empty"),
//             "--suri",
//             "//Alice",
//             "--url",
//             "https://rococo-contracts-rpc.polkadot.io"

// cargo contract call --contract 5E3rCrW49guH5V9HMrBqys76nSgkzneaLvRvupZzT7DEoqj9 --message get_assistant --suri //Alice --dry-run
// cargo contract call --contract 5Ct5SP6D2viSQJ4HQimdnPCTzSGT71Md9FGGoMo1z3jVjQua  --message create_new_account --args 0x6469643a73616d3a726f6f743a6a686a627366676975736966373875776e39733938306839 0x3332333233323332 0x656d707479 --suri //Alice "blast they annual column pave umbrella again olympic exotic vibrant lemon visa" --url wss://rococo-contracts-rpc.polkadot.io
// 5Ct5SP6D2viSQJ4HQimdnPCTzSGT71Md9FGGoMo1z3jVjQua

fn test_get_file_info() -> String {
    let did = "did:sam:root:jhjbsfgiusif78uwn9s980h9";
    let output = Command::new("cargo")
        .args([
            "contract",
            "call",
            "--contract",
            "5Ct5SP6D2viSQJ4HQimdnPCTzSGT71Md9FGGoMo1z3jVjQua",
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
            "5Ct5SP6D2viSQJ4HQimdnPCTzSGT71Md9FGGoMo1z3jVjQua",
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
            "5Ct5SP6D2viSQJ4HQimdnPCTzSGT71Md9FGGoMo1z3jVjQua",
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
            "5Ct5SP6D2viSQJ4HQimdnPCTzSGT71Md9FGGoMo1z3jVjQua",
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
            "5Ct5SP6D2viSQJ4HQimdnPCTzSGT71Md9FGGoMo1z3jVjQua",
            "--message",
            "account_is_auth",
            "--args",
            &str_to_hex("did:sam:root:jdlksdfksfuwn9s980h9"),
            &str_to_hex(&format!("{}", 323232)),
            "--suri",
            "//Alice",
            // "blast they annual column pave umbrella again olympic exotic vibrant lemon visa",
            "--dry-run",
            // "--url",
            // "wss://rococo-contracts-rpc.polkadot.io",
        ])
        .current_dir("../sam_os")
        .output()
        .expect("failed to execute process");

    let binding = String::from_utf8_lossy(&output.stdout);
    println!("{}", binding);
    if parse_contract_boolean(&binding) {
        println!("The account exists!");
    } else {
        println!("The account doesn't exists!")
    }

    println!("{}", parse_contract_return_data(&binding));
    println!("The nonce is {}", parse_contract_last_u64(&binding));
}

fn test_hashtable_update() {
    // update_hashtable(cfg: &Arc<Config>, cid: &str, did: &str, curr_timestamp: u64);
    let output = Command::new("cargo")
        .args([
            "contract",
            "call",
            "--contract",
            "5Ct5SP6D2viSQJ4HQimdnPCTzSGT71Md9FGGoMo1z3jVjQua",
            "--message",
            "update_hashtable",
            "--args",
            &str_to_hex("dhkjalkhc89ar90rcaroc0kfkajkjf00ad"),
            &str_to_hex("did:sam:root:jdlksdfksfuwn9s980h9"),
            &format!("{}", 920099999),
            "--suri",
            "//Alice",
            "--skip-confirm",
        ])
        .current_dir("../sam_os")
        .output()
        .expect("failed to execute process");


    println!("{:#?}", output);

    println!("success? {}", output.status.success())
}

fn test_account_creation() {
    let output = Command::new("cargo")
        .args([
            "contract",
            "call",
            "--contract",
            "5Ct5SP6D2viSQJ4HQimdnPCTzSGT71Md9FGGoMo1z3jVjQua",
            "--message",
            "create_new_account",
            "--args",
            &str_to_hex("did:sam:root:jdlksdfksfuwn9s980h9"),
            &str_to_hex(&format!("{}", 323232)),
            &str_to_hex("empcty"),
            &str_to_hex("hashtable_uri"),
            "--suri",
            // "blast they annual column pave umbrella again olympic exotic vibrant lemon visa",
            // "--url",
            // "wss://rococo-contracts-rpc.polkadot.io",
            "//Alice",
            // "--dry-run"
            "--skip-confirm",
        ])
        .current_dir("../sam_os")
        .output()
        .expect("failed to execute process");

    println!("{:#?}", output);

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

pub fn parse_contract_last_u64(binding: &str) -> u64 {
    let chunks = binding.split("UInt(").collect::<Vec<&str>>();
    let last_chunk: String = chunks[chunks.len() - 1]
        .to_owned()
        .as_str()
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect();
    last_chunk.parse::<u64>().unwrap_or_default()
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

// testing IPFS pulling
fn pull_from_ipfs(cid: &str) -> (bool, String) {
    let output = Command::new("ipfs")
        .args(["cat", cid])
        .output()
        .expect("failed to read data from node");

    let result = String::from_utf8_lossy(&output.stdout);
    println!("ipfs file download, cid: {}", cid);
    println!("result: {}", result);

    (output.status.success(), result.into())
}
