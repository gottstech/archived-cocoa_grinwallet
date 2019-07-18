// Copyright 2019 Ivan Sorokin.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Libs Wallet External API Definition

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use grin_wallet_api::{Foreign, Owner};
use grin_wallet_config::WalletConfig;
use grin_wallet_impls::{
    instantiate_wallet, Error, ErrorKind, FileWalletCommAdapter, HTTPNodeClient,
    HTTPWalletCommAdapter, LMDBBackend, WalletSeed,
};
use grin_wallet_libwallet::api_impl::types::InitTxArgs;
use grin_wallet_libwallet::{NodeClient, WalletInst};
use grin_wallet_util::grin_core::global::ChainTypes;
use grin_wallet_util::grin_keychain::ExtKeychain;
use grin_wallet_util::grin_util::{file::get_first_line, Mutex};

/// Default minimum confirmation
pub const MINIMUM_CONFIRMATIONS: u64 = 10;

fn cstr_to_str(s: *const c_char) -> String {
    unsafe { CStr::from_ptr(s).to_string_lossy().into_owned() }
}

#[no_mangle]
pub extern "C" fn cstr_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        // Recover the CString so rust can deallocate it
        CString::from_raw(s)
    };
}

unsafe fn result_to_cstr(res: Result<String, Error>, error: *mut u8) -> *const c_char {
    match res {
        Ok(res) => {
            *error = 0;
            CString::new(res).unwrap().into_raw()
        }
        Err(e) => {
            *error = 1;
            CString::new(serde_json::to_string(&format!("{}", e)).unwrap())
                .unwrap()
                .into_raw()
        }
    }
}

unsafe fn result2_to_cstr(res: Result<(bool, String), Error>, error: *mut u8) -> *const c_char {
    match res {
        Ok((validated, res)) => {
            if validated {
                *error = 0;
            } else {
                *error = 2;
            }
            CString::new(res).unwrap().into_raw()
        }
        Err(e) => {
            *error = 1;
            CString::new(serde_json::to_string(&format!("{}", e)).unwrap())
                .unwrap()
                .into_raw()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct MobileWalletCfg {
    account: String,
    chain_type: String,
    data_dir: String,
    node_api_addr: String,
    password: String,
    minimum_confirmations: u64,
}

impl MobileWalletCfg {
    pub fn from_str(json_cfg: &str) -> Result<Self, Error> {
        serde_json::from_str::<MobileWalletCfg>(json_cfg)
            .map_err(|e| Error::from(ErrorKind::GenericError(e.to_string())))
    }
}

fn new_wallet_config(config: MobileWalletCfg) -> Result<WalletConfig, Error> {
    let chain_type = match config.chain_type.as_str() {
        "mainnet" => ChainTypes::Mainnet,
        "floonet" => ChainTypes::Floonet,
        _ => {
            return Err(Error::from(ErrorKind::GenericError(
                "unsupported chain type".to_owned(),
            )));
        }
    };

    Ok(WalletConfig {
        chain_type: Some(chain_type),
        api_listen_interface: "127.0.0.1".to_string(),
        api_listen_port: 3415,
        api_secret_path: Some(".api_secret".to_string()),
        node_api_secret_path: Some(config.data_dir.clone() + "/.api_secret"),
        check_node_api_http_addr: config.node_api_addr,
        data_file_dir: config.data_dir + "/wallet_data",
        tls_certificate_file: None,
        tls_certificate_key: None,
        dark_background_color_scheme: Some(true),
        keybase_notify_ttl: Some(1440),
        no_commit_cache: Some(false),
        owner_api_include_foreign: Some(false),
        owner_api_listen_port: Some(3420),
    })
}

fn check_password(json_cfg: &str, password: &str) -> Result<String, Error> {
    let wallet_config = new_wallet_config(MobileWalletCfg::from_str(json_cfg)?)?;
    WalletSeed::from_file(&wallet_config.data_file_dir, password).map_err(|e| Error::from(e))?;
    Ok("OK".to_owned())
}

#[no_mangle]
pub extern "C" fn grin_check_password(
    json_cfg: *const c_char,
    password: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = check_password(&cstr_to_str(json_cfg), &cstr_to_str(password));
    unsafe { result_to_cstr(res, error) }
}

fn init_wallet_seed() -> Result<String, Error> {
    WalletSeed::init_new(32).to_mnemonic()
}

#[no_mangle]
pub extern "C" fn grin_init_wallet_seed(error: *mut u8) -> *const c_char {
    let res = init_wallet_seed();
    unsafe { result_to_cstr(res, error) }
}

fn wallet_init(json_cfg: &str, password: &str) -> Result<String, Error> {
    let wallet_config = new_wallet_config(MobileWalletCfg::from_str(json_cfg)?)?;
    let node_api_secret = get_first_line(wallet_config.node_api_secret_path.clone());
    let seed = WalletSeed::init_file(&wallet_config.data_file_dir, 32, None, password, false)?;
    let node_client = HTTPNodeClient::new(&wallet_config.check_node_api_http_addr, node_api_secret);
    let _: LMDBBackend<HTTPNodeClient, ExtKeychain> =
        LMDBBackend::new(wallet_config, password, node_client)?;
    seed.to_mnemonic()
}

#[no_mangle]
pub extern "C" fn grin_wallet_init(
    json_cfg: *const c_char,
    password: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = wallet_init(&cstr_to_str(json_cfg), &cstr_to_str(password));
    unsafe { result_to_cstr(res, error) }
}

fn wallet_init_recover(json_cfg: &str, mnemonic: &str) -> Result<String, Error> {
    let config = MobileWalletCfg::from_str(json_cfg)?;
    let wallet_config = new_wallet_config(config.clone())?;
    WalletSeed::recover_from_phrase(&wallet_config.data_file_dir, mnemonic, config.password.as_str())?;
    let node_api_secret = get_first_line(wallet_config.node_api_secret_path.clone());
    let node_client = HTTPNodeClient::new(&wallet_config.check_node_api_http_addr, node_api_secret);
    let _: LMDBBackend<HTTPNodeClient, ExtKeychain> =
        LMDBBackend::new(wallet_config, config.password.as_str(), node_client)?;
    Ok("OK".to_owned())
}

#[no_mangle]
pub extern "C" fn grin_wallet_init_recover(
    json_cfg: *const c_char,
    mnemonic: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = wallet_init_recover(
        &cstr_to_str(json_cfg),
        &cstr_to_str(mnemonic),
    );
    unsafe { result_to_cstr(res, error) }
}

fn wallet_restore(
    json_cfg: &str,
    start_index: u64,
    batch_size: u64,
) -> Result<String, Error> {
    let config = MobileWalletCfg::from_str(json_cfg)?;
    let wallet_config = new_wallet_config(config.clone())?;
    let node_api_secret = get_first_line(wallet_config.node_api_secret_path.clone());
    let node_client = HTTPNodeClient::new(&wallet_config.check_node_api_http_addr, node_api_secret);
    let wallet = instantiate_wallet(wallet_config, node_client, config.password.as_str(), &config.account)?;
    let api = Owner::new(wallet.clone());

    let (highest_index, last_retrieved_index, num_of_found) = api
        .restore_batch(start_index, batch_size)
        .map_err(|e| Error::from(e))?;
    Ok(json!({
        "highestIndex": highest_index,
        "lastRetrievedIndex": last_retrieved_index,
        "numberOfFound": num_of_found,
    })
    .to_string())
}

#[no_mangle]
pub extern "C" fn grin_wallet_restore(
    json_cfg: *const c_char,
    start_index: u64,
    batch_size: u64,
    error: *mut u8,
) -> *const c_char {
    let res = wallet_restore(
        &cstr_to_str(json_cfg),
        start_index,
        batch_size,
    );
    unsafe { result_to_cstr(res, error) }
}

fn wallet_check(
    json_cfg: &str,
    start_index: u64,
    batch_size: u64,
    update_outputs: bool,
) -> Result<String, Error> {
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);
    let (highest_index, last_retrieved_index) = api
        .check_repair_batch(true, start_index, batch_size, update_outputs)
        .map_err(|e| Error::from(e))?;

    Ok(json!({
        "highestIndex": highest_index,
        "lastRetrievedIndex": last_retrieved_index,
    })
    .to_string())
}

#[no_mangle]
pub extern "C" fn grin_wallet_check(
    json_cfg: *const c_char,
    start_index: u64,
    batch_size: u64,
    update_outputs: bool,
    error: *mut u8,
) -> *const c_char {
    let res = wallet_check(
        &cstr_to_str(json_cfg),
        start_index,
        batch_size,
        update_outputs,
    );
    unsafe { result_to_cstr(res, error) }
}

fn get_wallet_mnemonic(json_cfg: &str) -> Result<String, Error> {
    let config = MobileWalletCfg::from_str(json_cfg)?;
    let wallet_config = new_wallet_config(config.clone())?;
    let seed = WalletSeed::from_file(&wallet_config.data_file_dir, config.password.as_str())?;
    seed.to_mnemonic()
}

#[no_mangle]
pub extern "C" fn grin_get_wallet_mnemonic(
    json_cfg: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = get_wallet_mnemonic(&cstr_to_str(json_cfg));
    unsafe { result_to_cstr(res, error) }
}

fn get_wallet_instance(
    config: MobileWalletCfg,
) -> Result<Arc<Mutex<WalletInst<impl NodeClient, ExtKeychain>>>, Error> {
    let wallet_config = new_wallet_config(config.clone())?;
    let node_api_secret = get_first_line(wallet_config.node_api_secret_path.clone());
    let node_client = HTTPNodeClient::new(&wallet_config.check_node_api_http_addr, node_api_secret);

    instantiate_wallet(
        wallet_config,
        node_client,
        config.password.as_str(),
        config.account.as_str(),
    )
}

fn get_balance(json_cfg: &str) -> Result<(bool, String), Error> {
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);
    let (validated, wallet_info) = api.retrieve_summary_info(true, MINIMUM_CONFIRMATIONS)?;
    Ok((validated, serde_json::to_string(&wallet_info).unwrap()))
}

#[no_mangle]
pub extern "C" fn grin_get_balance(
    json_cfg: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = get_balance(&cstr_to_str(json_cfg));
    unsafe { result2_to_cstr(res, error) }
}

fn tx_retrieve(json_cfg: &str, tx_slate_id: &str) -> Result<String, Error> {
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);
    let uuid = Uuid::parse_str(tx_slate_id).map_err(|e| ErrorKind::GenericError(e.to_string()))?;
    let txs = api.retrieve_txs(true, None, Some(uuid))?;
    Ok(serde_json::to_string(&txs).unwrap())
}

#[no_mangle]
pub extern "C" fn grin_tx_retrieve(
    json_cfg: *const c_char,
    tx_slate_id: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = tx_retrieve(
        &cstr_to_str(json_cfg),
        &cstr_to_str(tx_slate_id),
    );
    unsafe { result_to_cstr(res, error) }
}

fn txs_retrieve(json_cfg: &str) -> Result<String, Error> {
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);

    match api.retrieve_txs(true, None, None) {
        Ok(txs) => Ok(serde_json::to_string(&txs).unwrap()),
        Err(e) => Err(Error::from(e)),
    }
}

#[no_mangle]
pub extern "C" fn grin_txs_retrieve(
    state_json: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = txs_retrieve(&cstr_to_str(state_json));
    unsafe { result_to_cstr(res, error) }
}

fn outputs_retrieve(json_cfg: &str, tx_id: Option<u32>) -> Result<String, Error> {
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);
    let outputs = api.retrieve_outputs(true, true, tx_id)?;
    Ok(serde_json::to_string(&outputs).unwrap())
}

#[no_mangle]
pub extern "C" fn grin_output_retrieve(
    json_cfg: *const c_char,
    tx_id: u32,
    error: *mut u8,
) -> *const c_char {
    let res = outputs_retrieve(&cstr_to_str(json_cfg), Some(tx_id));
    unsafe { result_to_cstr(res, error) }
}

#[no_mangle]
pub extern "C" fn grin_outputs_retrieve(
    json_cfg: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = outputs_retrieve(&cstr_to_str(json_cfg), None);
    unsafe { result_to_cstr(res, error) }
}

fn init_send_tx(
    json_cfg: &str,
    amount: u64,
    selection_strategy: &str,
    target_slate_version: Option<u16>,
    message: &str,
) -> Result<String, Error> {
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);
    let tx_args = InitTxArgs {
        src_acct_name: None,
        amount,
        minimum_confirmations: MINIMUM_CONFIRMATIONS,
        max_outputs: 500,
        num_change_outputs: 1,
        selection_strategy: selection_strategy.to_string(),
        message: Some(message.to_string()),
        target_slate_version,
        estimate_only: None,
        send_args: None,
    };
    let slate = api.init_send_tx(tx_args)?;
    api.tx_lock_outputs(&slate, 0)?;
    Ok(serde_json::to_string(&slate).expect("fail to serialize slate to json string"))
}

#[no_mangle]
pub extern "C" fn grin_init_tx(
    json_cfg: *const c_char,
    amount: u64,
    selection_strategy: *const c_char,
    target_slate_version: i16,
    message: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let mut slate_version: Option<u16> = None;
    if target_slate_version >= 0 {
        slate_version = Some(target_slate_version as u16);
    }

    let res = init_send_tx(
        &cstr_to_str(json_cfg),
        amount,
        &cstr_to_str(selection_strategy),
        slate_version,
        &cstr_to_str(message),
    );
    unsafe { result_to_cstr(res, error) }
}

fn send_tx(
    json_cfg: &str,
    amount: u64,
    receiver_wallet_url: &str,
    selection_strategy: &str,
    target_slate_version: Option<u16>,
    message: &str,
) -> Result<String, Error> {
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);
    let args = InitTxArgs {
        src_acct_name: None,
        amount,
        minimum_confirmations: MINIMUM_CONFIRMATIONS,
        max_outputs: 500,
        num_change_outputs: 1,
        selection_strategy: selection_strategy.to_string(),
        message: Some(message.to_string()),
        target_slate_version,
        estimate_only: None,
        send_args: None,
    };
    let slate = api.init_send_tx(args)?;
    api.tx_lock_outputs(&slate, 0)?;

    let adapter = HTTPWalletCommAdapter::new();
    match adapter.send_tx_sync(receiver_wallet_url, &slate) {
        Ok(mut slate) => {
            api.verify_slate_messages(&slate)?;
            api.finalize_tx(&mut slate)?;
            Ok(serde_json::to_string(&slate).expect("fail to serialize slate to json string"))
        }
        Err(e) => {
            api.cancel_tx(None, Some(slate.id))?;
            Err(Error::from(e))
        }
    }
}

#[no_mangle]
pub extern "C" fn grin_send_tx(
    json_cfg: *const c_char,
    amount: u64,
    receiver_wallet_url: *const c_char,
    selection_strategy: *const c_char,
    target_slate_version: i16,
    message: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let mut slate_version: Option<u16> = None;
    if target_slate_version >= 0 {
        slate_version = Some(target_slate_version as u16);
    }

    let res = send_tx(
        &cstr_to_str(json_cfg),
        amount,
        &cstr_to_str(receiver_wallet_url),
        &cstr_to_str(selection_strategy),
        slate_version,
        &cstr_to_str(message),
    );
    unsafe { result_to_cstr(res, error) }
}

fn cancel_tx(json_cfg: &str, tx_slate_id: &str) -> Result<String, Error> {
    let uuid = Uuid::parse_str(tx_slate_id).map_err(|e| ErrorKind::GenericError(e.to_string()))?;
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);
    api.cancel_tx(None, Some(uuid))?;
    Ok("OK".to_owned())
}

#[no_mangle]
pub extern "C" fn grin_cancel_tx(
    json_cfg: *const c_char,
    tx_slate_id: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = cancel_tx(&cstr_to_str(json_cfg), &cstr_to_str(tx_slate_id));
    unsafe { result_to_cstr(res, error) }
}

fn post_tx(json_cfg: &str, tx_slate_id: &str) -> Result<String, Error> {
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);
    let uuid = Uuid::parse_str(tx_slate_id).map_err(|e| ErrorKind::GenericError(e.to_string()))?;
    let (validated, txs) = api.retrieve_txs(true, None, Some(uuid))?;
    if txs[0].confirmed {
        return Err(Error::from(ErrorKind::GenericError(format!(
            "Transaction already confirmed"
        ))));
    } else if !validated {
        return Err(Error::from(ErrorKind::GenericError(format!(
            "api.retrieve_txs not validated"
        ))));
    }

    let stored_tx = api.get_stored_tx(&txs[0])?;
    match stored_tx {
        Some(stored_tx) => {
            api.post_tx(&stored_tx, true)?;
            Ok("OK".to_owned())
        }
        None => Err(Error::from(ErrorKind::GenericError(format!(
            "transaction data not found"
        )))),
    }
}

#[no_mangle]
pub extern "C" fn grin_post_tx(
    json_cfg: *const c_char,
    tx_slate_id: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = post_tx(
        &cstr_to_str(json_cfg),
        &cstr_to_str(tx_slate_id),
    );
    unsafe { result_to_cstr(res, error) }
}

fn tx_file_receive(
    json_cfg: &str,
    slate_file_path: &str,
    message: &str,
) -> Result<String, Error> {
    let config = MobileWalletCfg::from_str(json_cfg)?;
    let wallet = get_wallet_instance(config.clone())?;
    let api = Foreign::new(wallet, None);
    let adapter = FileWalletCommAdapter::new();
    let mut slate = adapter.receive_tx_async(&slate_file_path)?;
    api.verify_slate_messages(&slate)?;
    slate = api.receive_tx(&slate, Some(&config.account), Some(message.to_string()))?;
    Ok(serde_json::to_string(&slate).expect("fail to serialize slate to json string"))
}

#[no_mangle]
pub extern "C" fn grin_tx_file_receive(
    json_cfg: *const c_char,
    slate_file_path: *const c_char,
    message: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = tx_file_receive(
        &cstr_to_str(json_cfg),
        &cstr_to_str(slate_file_path),
        &cstr_to_str(message),
    );
    unsafe { result_to_cstr(res, error) }
}

fn tx_file_finalize(
    json_cfg: &str,
    slate_file_path: &str,
) -> Result<String, Error> {
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);
    let adapter = FileWalletCommAdapter::new();
    let mut slate = adapter.receive_tx_async(slate_file_path)?;
    api.verify_slate_messages(&slate)?;
    slate = api.finalize_tx(&slate)?;
    Ok(serde_json::to_string(&slate).expect("fail to serialize slate to json string"))
}

#[no_mangle]
pub extern "C" fn grin_tx_file_finalize(
    json_cfg: *const c_char,
    slate_file_path: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = tx_file_finalize(
        &cstr_to_str(json_cfg),
        &cstr_to_str(slate_file_path),
    );
    unsafe { result_to_cstr(res, error) }
}

fn chain_height(json_cfg: &str) -> Result<String, Error> {
    let wallet = get_wallet_instance(MobileWalletCfg::from_str(json_cfg)?)?;
    let api = Owner::new(wallet);
    let height = api.node_height()?;
    Ok(serde_json::to_string(&height).unwrap())
}

#[no_mangle]
pub extern "C" fn grin_chain_height(
    json_cfg: *const c_char,
    error: *mut u8,
) -> *const c_char {
    let res = chain_height(&cstr_to_str(json_cfg));
    unsafe { result_to_cstr(res, error) }
}
