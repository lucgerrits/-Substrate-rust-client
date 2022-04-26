#[macro_use]
extern crate clap;

mod common;
use clap::App;
use keyring::AccountKeyring;
#[allow(unused_imports)]
// use node_template_runtime::{BalancesCall, Call, Header};
use sp_core::crypto::Pair;
use sp_core::sr25519;
// use sp_runtime::MultiAddress;
use substrate_api_client::rpc::WsRpcClient;
#[allow(unused_imports)]
use substrate_api_client::{
    compose_extrinsic, compose_extrinsic_offline, Api, GenericAddress, UncheckedExtrinsicV4,
    XtStatus, ApiResult
};
use sp_core::H256;
// use std::str;

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    let from = AccountKeyring::Alice.pair();
    let client = WsRpcClient::new(&url);
    let init_api = Api::<sr25519::Pair, _>::new(client).map(|api| api.set_signer(from));

    match init_api {
        Ok(_) => println!("API OK"),
        Err(e) => return eprintln!("API ERROR: {}", e),
    };
    let api = init_api.unwrap();

    // Information for Era for mortal transactions
    // let head = api.get_finalized_head().unwrap().unwrap();
    // let h: Header = api.get_header(Some(head)).unwrap().unwrap();
    // let period = 5;

    // common::print_meta(api.clone());

    let key = "foo"; //0x666F6F
    let val = "bar"; //0x626172
    println!(
        "Set key: {:X?} <=> {:X?}",
        key.clone(),
        key.clone().as_bytes().to_vec()
    );
    println!(
        "Set value: {:X?} <=> {:X?}",
        val.clone(),
        val.clone().as_bytes().to_vec()
    );

    ///////////// one way:
    // define the recipient
    // let to = MultiAddress::Id(AccountKeyring::Bob.to_account_id());
    // let xt: UncheckedExtrinsicV4<_> = compose_extrinsic_offline!(
    //     api.clone().signer.unwrap(),
    //     Call::Balances(BalancesCall::transfer {
    //         dest: to.clone(),
    //         value: 42
    //     }),
    //     api.get_nonce().unwrap(),
    //     Era::mortal(period, h.number.into()),
    //     api.genesis_hash,
    //     head,
    //     api.runtime_version.spec_version,
    //     api.runtime_version.transaction_version
    // );

    let res = set_storage_value(
        api.clone(),
        key.clone().as_bytes().to_vec(),
        val.clone().as_bytes().to_vec(),
    );
    match res {
        Err(e) => eprintln!("ERROR: {}", e),
        Ok(blockh) => println!("[+] Transaction got included in block {:?}", blockh)
    }
}

/// Set storage data value with a given key
/// # Arguments
/// * `api` - Api endpoint
/// * `key` - Storage key
/// * `value` - Storage value
fn set_storage_value(
    api: Api<sr25519::Pair, WsRpcClient>,
    key: Vec<u8>,
    value: Vec<u8>,
) -> ApiResult<Option<H256>> {
    // set the storage
    // define the recipient
    let xt: UncheckedExtrinsicV4<_> =
        compose_extrinsic!(api.clone(), "KeyvalueModule", "store", key, value);
    ///////////// send the tx:
    api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("../src/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}", url);
    url
}
