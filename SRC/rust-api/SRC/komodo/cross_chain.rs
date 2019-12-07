#![warn(missing_docs)]
//! 
//! This is the documentation for 'Cross-Chain API' module of Komodo.
//!
//! The 'Cross-Chain API' module of Komodo contains functionality of the 'Cross-Chain API' noted on the
//! [Komodo website].
//! 
//! # Remarks
//! 
//! * ? Some documentation of methods may reference a different method.
//! 
//! * A valid KomodoRPC object type must be passed in.
//! 
//! * All examples for each method assumes a valid KomodoRPC object, named `some_user`, is used.
//! 
//! * Use of any methods requires the following modules:
//! ```
//! mod komodorpcutil;
//! mod komodo;
//! use komodorpcutil::KomodoRPC;
//! ```
//! 
//! [Komodo website]: https://docs.komodoplatform.com/basic-docs/smart-chains/smart-chain-api/crosschain.html
//! 


// TODO: - run fmt and clippy
//       - document all methods
//          - more advanced examples
//       - ? potential fix for no function/method overloading if extra time
//              -> trait object static/dynamic dispatch for multi-type parameter
//       - ? some_user parameter may be simplified further


use super::komodorpcutil;
//use komodorpcutil::KomodoRPC;

/// ?? The migrate_create_burn_transaction method creates a transaction burning a specific amount of 
/// coins or tokens. This method also creates a payouts object which is later used to create an 
/// import transaction for the value corresponding to the burned amount. This method should be 
/// called on the source chain.
/// 
/// The method creates a burn transaction and returns it. This should be broadcast to the source chain using the sendrawtransaction method. After the burn transaction is successfully mined, the user might have to wait for some amount of time for the back notarization to reach the source chain. The back notarization contains the MoMoM fingerprints of the mined block that contains the burn transaction.
/// 
/// The hex value of the burn transaction along with the other returned value payouts are used as arguments for the migrate_createimporttransaction method.
/// 
/// # Arguments
/// 
/// * `some_user` - A required KomodoRPC object type that represents the user.
/// * `dest_chain` - A required string type that represents the name of the destination chain.
/// * `dest_address` - A required string that represents the address on the destination chain where coins are to be sent; the pubkey if tokens are to be sent.
/// * `amount` - A required u32 that represents the amount in coins or tokens that should be burned on the source chain and created on the destination chain; if the indicated assets are tokens, the amount can be set only to 1, as only migration of non-fungible tokens are supported at this time.
/// * `token_id` - An optional string that represents the token id in hex; if set, the software assumes that the user is migrating tokens.
/// 
/// # Response
/// 
/// * `payouts` - A hex string of the created payouts; this value is passed into the migrate_createimporttransaction method.
/// * `BurnTxHex` - A hex string of the returned burn transaction.
/// 
/// # Examples
/// ```
/// 
/// let result = komodo::cross_chain::migrate_create_burn_transaction(some_user, 
///                                                                   "CFEKDRAGON".to_string(), 
///                                                                   "RBQ1XwmzduHvciRJbXbWY9YBSNtaqZvfC4".to_string(), 
///                                                                   7.77u32, 
///                                                                   None);
/// 
/// ```
/// 
/// TODO: Provide more advanced examples
/// 
pub fn migrate_create_burn_transaction(
    some_user: komodorpcutil::KomodoRPC, 
    dest_chain: String,
    dest_address: String,
    amount: f64,
    token_id: Option<String>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("migrate_createburntransaction");
    let method_body: String;
    let temp_token_id: String = token_id.unwrap_or("".to_string());
    
    // user provides token to migrate
    if !temp_token_id.is_empty()
    {
        
        //method_body = format!("[\"{0}\",\"{1}\",\"{2}\",\"{3}\"]", dest_chain, dest_address, amount, temp_token_id);
        method_body = String::from("[\"")
                        + &dest_chain.to_string()
                        + &String::from("\",\"")
                        + &dest_address.to_string()
                        + &String::from("\",")
                        + &amount.to_string()
                        + &String::from(",\"")
                        + &temp_token_id.to_string()
                        + &String::from("\"]");
        
    }
    
    // user did not provide token
    else
    {
        
        //method_body = format!("[\"{0}\",\"{1}\",\"{2}\"]", dest_chain, dest_address, amount);
        method_body = String::from("[\"")
                        + &dest_chain.to_string()
                        + &String::from("\",\"")
                        + &dest_address.to_string()
                        + &String::from("\",\"")
                        + &amount.to_string()
                        + &String::from("\"]");
            
    }
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    return result;
    
}


pub fn migrate_convert_to_export(
    some_user: komodorpcutil::KomodoRPC, 
    burn_tx: String,
    dest_chain: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("migrate_converttoexport");
    //let method_body = format!("[\"{0}\",\"{1}\"]", burn_tx, dest_chain);
    let method_body: String = String::from("[\"")
                    + &burn_tx.to_string()
                    + &String::from("\",\"")
                    + &dest_chain.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    return result;
    
}

pub fn migrate_create_import_transaction(
    some_user: komodorpcutil::KomodoRPC, 
    burn_tx: String,
    payouts: String,
    notary_tx_id1: Option<String>,
    notary_tx_idN: Option<String>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("migrate_createimporttransaction");
    let temp_notary_tx_id1: String = notary_tx_id1.unwrap_or("".to_string());
    let temp_notary_tx_idN: String = notary_tx_idN.unwrap_or("".to_string());
    
    let mut method_body: String = String::from("[\"")
                    + &burn_tx.to_string()
                    + &String::from("\",\"")
                    + &payouts.to_string();
    
    // concatentate string for optional strings
    if !temp_notary_tx_id1.is_empty()
    {
        
        method_body = method_body
                        + &String::from("\",\"")
                        + &temp_notary_tx_id1.to_string();
        
        // we want to build up the 'notary_tx_idN' parameter if it exists
        if !temp_notary_tx_idN.is_empty()
        {
            
            method_body = method_body
                            + &String::from("\",\"")
                            + &temp_notary_tx_idN.to_string();
            
        }
        
    }
    
    else if !temp_notary_tx_idN.is_empty()
    {
        
        method_body = method_body
                        + &String::from("\",\"")
                        + &temp_notary_tx_idN.to_string();
        
    }
    
    
    method_body = method_body + &String::from("\"]");
    
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    return result;
    
}


pub fn migrate_complete_import_transaction(
    some_user: komodorpcutil::KomodoRPC, 
    import_tx: String,
    offset: Option<String>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("migrate_completeimporttransaction");
    let temp_offset: String = offset.unwrap_or("".to_string());
    let mut method_body: String = String::from("[\"")
                    + &import_tx.to_string();
    
    if !temp_offset.is_empty()
    {
        
        method_body = method_body
                        + &String::from("\",\"")
                        + &temp_offset.to_string();
        
    }
    
    method_body = method_body + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

pub fn migrate_check_burn_transaction_source(
    some_user: komodorpcutil::KomodoRPC, 
    burn_tx_id: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("migrate_checkburntransactionsource");
    
    let method_body: String = String::from("[\"")
                    + &burn_tx_id.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

pub fn migrate_create_notary_approval_transaction(
    some_user: komodorpcutil::KomodoRPC, 
    burn_tx_id: String,
    tx_out_proof: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("migrate_createnotaryapprovaltransaction");
    
    let method_body: String = String::from("[\"")
                    + &burn_tx_id.to_string()
                    + &String::from("\",\"")
                    + &tx_out_proof.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

pub fn self_import(
    some_user: komodorpcutil::KomodoRPC, 
    dest_address: String,
    amount: f64)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("selfimport");
    
    let method_body: String = String::from("[\"")
                    + &dest_address.to_string()
                    + &String::from("\",")
                    + &amount.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

pub fn calc_MoM(
    some_user: komodorpcutil::KomodoRPC, 
    height: u32,
    MoM_depth: u32)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("calc_MoM");
    
    let method_body: String = String::from("[\"")
                    + &height.to_string()
                    + &String::from("\",\"")
                    + &MoM_depth.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

pub fn MoMoM_data(
    some_user: komodorpcutil::KomodoRPC, 
    symbol: String,
    kmd_height: u32,
    cc_id: u32)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("MoMoMdata");
    
    let method_body: String = String::from("[\"")
                    + &symbol.to_string()
                    + &String::from("\",\"")
                    + &kmd_height.to_string()
                    + &String::from("\",\"")
                    + &cc_id.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

pub fn asset_chain_proof(
    some_user: komodorpcutil::KomodoRPC, 
    tx_id: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("assetchainproof");
    
    let method_body: String = String::from("[\"")
                                + &tx_id.to_string()
                                + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

// get_notarisations_for_block vs scan_notarisations_db
//  both has a required numeric parameter,
//  but get_notar... does not need quotes in param
//  and scan_notar... does need quotes in param
pub fn get_notarisations_for_block(
    some_user: komodorpcutil::KomodoRPC, 
    height: u32)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("getNotarisationsForBlock");
    
    let method_body: String = String::from("[")
                                + &height.to_string()
                                + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

// TODO: Fix string parsing for not empty
pub fn scan_notarisations_db(
    some_user: komodorpcutil::KomodoRPC, 
    block_height: u32,
    symbol: String,
    blocks_limit: Option<u32>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("scanNotarisationsDB");
    let temp_blocks_limit: String = blocks_limit.unwrap_or(0).to_string();
    let mut method_body: String = String::from("[\"")
                    + &block_height.to_string()
                    + &String::from("\",\"")
                    + &symbol.to_string();
    
    if !temp_blocks_limit.is_empty()
    {
        
        method_body = method_body
                        + &String::from("\",\"")
                        + &temp_blocks_limit.to_string();
        
    }
    
    method_body = method_body + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

// NOTE: Komodo doc requires string OR number - currently forcing string only
pub fn get_imports(
    some_user: komodorpcutil::KomodoRPC, 
    hash_or_height: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("getimports");
    let method_body: String = String::from("[\"")
                    + &hash_or_height.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

pub fn get_wallet_burn_transactions(
    some_user: komodorpcutil::KomodoRPC, 
    count: Option<u32>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("getwalletburntransactions");
    let temp_count = count.unwrap_or(10);
    let method_body: String = String::from("[")
                    + &temp_count.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}
