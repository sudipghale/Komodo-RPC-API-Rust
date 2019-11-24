
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
mod komodorpcutil;
mod komodo;
use komodorpcutil::KomodoRPC;

pub fn main()
{
    
    let someAddress = String::from("127.0.0.1");
    let somePortNum = 12345;
    let someReqMethod = String::from("POST");
    let someUserName = String::from("USER");
    let somePassword = String::from("PASW");
    let someJSONRPCVer = String::from("1.0");
    let someRPCReqID = String::from("curltest");
    // cargo doc --no-deps --open --document-private-items
    
    // Create KomodoRPC type (an 'object') with variables
    let someUser = KomodoRPC::new(someAddress, somePortNum, someReqMethod,someUserName, somePassword,someJSONRPCVer, someRPCReqID);
    
    
    
    // --------------------------------------------------------------------------------------------
    /*
    
    // ----------------------------------
    // Cross-Chain API Module tests
    // ----------------------------------
    
    let dest_chain: String = "dest_chain".to_string();
    let dest_address: String = "dest_address".to_string();
    let amount: f64 = 7.77;
    let burn_tx: String = "burn_tx".to_string();
    let payouts: String = "payouts".to_string();
    let notary = Some("notary".to_string());
    let notaryN = Some("notaryN".to_string());
    let import_tx: String = "import_tx".to_string();
    let offset = Some("offset".to_string());
    let burn_tx_id = "burn_tx_id".to_string();
    let tx_out_proof: String = "tx_out_proof".to_string();
    let height: u32 = 3;
    let MoM_depth: u32 = 2;
    let symbol: String = "symbol".to_string();
    let kmd_height: u32 = 5;
    let cc_id: u32 = 007;
    let tx_id: String = "tx_id".to_string();
    let block_height: u32 = 3;
    let blocks_limit = Some(8);
    let hash_or_height: String = "hash_or_height".to_string();
    let count = Some(9);
    
    // migrate_create_burn_transaction(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  dest_chain: String,
    //  dest_address: String,
    //  amount: f64,
    //  token_id: Option<String>)
    //  --- NOTE: CHANGED u32 TO f64 numeric type to support decimal numbers ---
    //  ---         User must now have '.0' at the end of integers 
    //let result_migrate_create_burn_transaction = komodo::cross_chain::migrate_create_burn_transaction(someUser.clone(), dest_chain, dest_address, amount, None);
    
    // migrate_convert_to_export(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  burn_tx: String,
    //  dest_chain: String)
    //let result_migrate_convert_to_export = komodo::cross_chain::migrate_convert_to_export(someUser.clone(), burn_tx, dest_chain);
    
    // migrate_create_import_transaction(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  burn_tx: String,
    //  payouts: String,
    //  notary_tx_id1: Option<String>,
    //  notary_tx_idN: Option<String>)
    //let result_migrate_create_import_transaction = komodo::cross_chain::migrate_create_import_transaction(someUser.clone(), burn_tx, payouts, notary, notaryN);
    
    // migrate_complete_import_transaction(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  import_tx: String,
    //  offset: Option<String>)
    //let result_migrate_complete_import_transaction = komodo::cross_chain::migrate_complete_import_transaction(someUser.clone(), import_tx, offset);
    
    // migrate_check_burn_transaction_source(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  burn_tx_id: String)
    //let result_migrate_check_burn_transaction_source = komodo::cross_chain::migrate_check_burn_transaction_source(someUser.clone(), burn_tx_id);
    
    // migrate_create_notary_approval_transaction(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  burn_tx_id: String,
    //  tx_out_proof: String)
    //let result_migrate_create_notary_approval_transaction = komodo::cross_chain::migrate_create_notary_approval_transaction(someUser.clone(), burn_tx_id, tx_out_proof);
    
    // self_import(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  dest_address: String,
    //  amount: f64)
    //let result_self_import = komodo::cross_chain::self_import(someUser.clone(), dest_address, amount);
    
    // calc_MoM(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  height: u32,
    //  MoM_depth: u32)
    //let result_calc_MoM = komodo::cross_chain::calc_MoM(someUser.clone(), height, MoM_depth);
    
    // MoMoM_data(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  symbol: String,
    //  kmd_height: u32,
    //  cc_id: u32)
    //let result_MoMoM_data = komodo::cross_chain::MoMoM_data(someUser.clone(), symbol, kmd_height, cc_id);
    
    // asset_chain_proof(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  tx_id: String)
    //let result_asset_chain_proof = komodo::cross_chain::asset_chain_proof(someUser.clone(), tx_id);
    
    // get_notarisations_for_block(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  height: u32)
    //let result_get_notarisations_for_block = komodo::cross_chain::get_notarisations_for_block(someUser.clone(), height);
    
    // scan_notarisations_db(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  block_height: u32,
    //  symbol: String,
    //  blocks_limit: Option<u32>)
    //let result_scan_notarisations_db = komodo::cross_chain::scan_notarisations_db(someUser.clone(), block_height, symbol, blocks_limit);
    
    // get_imports(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  hash_or_height: String)
    //let result_get_imports = komodo::cross_chain::get_imports(someUser.clone(), hash_or_height);
    
    // get_wallet_burn_transactions(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  count: Option<u32>)
    //let result_get_wallet_burn_transactions = komodo::cross_chain::get_wallet_burn_transactions(someUser.clone(), count);
    
    */
    // --------------------------------------------------------------------------------------------
    /*
    // ----------------------------------
    // Util Module tests
    // ----------------------------------
    
    let number_required: u32 = 1;
    let keys: String = "[\"KD\",\"TA\",\"SG\",\"MP\",\"NP\",\"DN\",\"NN\",\"AM\",\"VM\",\"AR\",\"GC\",\"NS\",\"LG\",\"KH\"]".to_string();
    let script_pub_key: String = "qpwoslxvmrhuvitjyzh".to_string();
    let n_blocks: u32 = 2;
    let hash: String = "2793b07280R38U998GU4W98WG89GW50".to_string();
    let tx_id: String = "3301b4032c3a0ca".to_string();
    let address: String = "address".to_string();
    let signature: String = "signature".to_string();
    let message: String = "msg".to_string();
    let addr: String = "asdf".to_string();
    
    // create_multisig(
    //  some_user: komodorpcutil::KomodoRPC, 
    //  number_required: u32,
    //  keys: String)
    //let result_create_multisig = komodo::util::create_multisig(someUser.clone(), number_required, keys);
    
    // decode_ccopret(
    //  some_user: komodorpcutil::KomodoRPC,
    //  script_pub_key: String)
    //let result_decode_ccopret = komodo::util::decode_ccopret(someUser.clone(), script_pub_key);
    
    // estimate_fee(
    //  some_user: komodorpcutil::KomodoRPC,
    //  n_blocks: u32)
    //let result_estimate_fee = komodo::util::estimate_fee(someUser.clone(), n_blocks);
    
    // estimate_priority(
    //  some_user: komodorpcutil::KomodoRPC,
    //  n_blocks: u32)
    //let result_estimate_priority = komodo::util::estimate_priority(someUser.clone(), n_blocks);
    
    // invalidate_block(
    //  some_user: komodorpcutil::KomodoRPC,
    //  hash: String)
    //let result_invalidate_block = komodo::util::invalidate_block(someUser.clone(), hash);
    
    // reconsider_block(
    //  some_user: komodorpcutil::KomodoRPC,
    //  hash: String)
    //let result_reconsider_block = komodo::util::reconsider_block(someUser.clone(), hash);
    
    // tx_notarized_confirmed(
    //  some_user: komodorpcutil::KomodoRPC,
    //  tx_id: String)
    //let result_tx_notarized_confirmed = komodo::util::tx_notarized_confirmed(someUser.clone(), tx_id);
    
    // validate_address(
    //  some_user: komodorpcutil::KomodoRPC,
    //  address: String)
    //let result_validate_address = komodo::util::validate_address(someUser.clone(), address);
    
    // verify_message(
    //  some_user: komodorpcutil::KomodoRPC,
    //  address: String,
    //  signature: String,
    //  message: String)
    //let result_verify_message = komodo::util::verify_message(someUser.clone(), address, signature, message);
    
    // z_validate_address(
    //  some_user: komodorpcutil::KomodoRPC,
    //  z_addr: String)
    //let result_ = komodo::util::z_validate_address(someUser.clone(), z_addr);
    
    */
    // --------------------------------------------------------------------------------------------
    
}