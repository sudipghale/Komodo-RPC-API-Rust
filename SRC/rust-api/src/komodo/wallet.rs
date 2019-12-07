#![warn(missing_docs)]
//! 
//! This is the documentation for 'Wallet' module of Komodo.
//!
//! The 'Wallet' module of Komodo contains functionality of the 'Wallet' noted on the
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
//! [Komodo website]: https://docs.komodoplatform.com/basic-docs/smart-chains/smart-chain-api/wallet.html
//! 



// TODO: - run fmt and clippy
//       - document all methods
//          - more advanced examples
//       - ? potential fix for no function/method overloading if extra time
//              -> trait object static/dynamic dispatch for multi-type parameter
//       - ? some_user parameter may be simplified further



use super::komodorpcutil;
//use komodorpcutil::KomodoRPC;



// The move method in Wallet module has been deprecated.



pub fn resend_wallet_transactions(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("resendwallettransactions");
    
    let method_body: String = String::from("[]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


// The sendfrom method in Wallet module has been deprecated.


pub fn send_many(
    some_user: komodorpcutil::KomodoRPC, 
    account: String,
    amounts: String,
    minconf: Option<u32>,
    comment: Option<String>,
    subtract_fee_from_amount: Option<String>,
    address: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("sendmany");
    let temp_minconf = minconf.unwrap_or(1);
    let temp_comment: String = comment.unwrap_or("");
    let temp_subtract_fee_from_amount: String = subtract_fee_from_amount.unwrap_or("".to_string());
    // dont use account?
    let method_body: String = String::from("[\"")
                    + &String::from("\",")
                    + &temp_minconf.to_string()
                    + &String::from(",")
                    + &String::from("\"")
                    + &temp_comment.to_string()
                    + &String::from("\"")
                    + &String::from(",")
                    + &temp_subtract_fee_from_amount.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn send_to_address(
    some_user: komodorpcutil::KomodoRPC, 
    komodo_address: String,
    amount: f64,
    comment: Option<String>,
    comment_to: Option<String>,
    subtract_fee_from_amount: Option<bool>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("sendtoaddress");
    let temp_comment: String = comment.unwrap_or("");
    let temp_comment_to: String = comment_to.unwrap_or("");
    let temp_subtract_fee_from_amount = subtract_fee_from_amount.unwrap_or(false);
    
    let method_body: String = String::from("[\"")
                    + &komodo_address.to_string()
                    + &String::from("\",")
                    + &amount.to_string()
                    + &String::from(",\"")
                    + &temp_comment.to_string()
                    + &String::from("\",")
                    + &temp_subtract_fee_from_amount.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


// The setaccount method in Wallet module has been deprecated.


pub fn set_pub_key(
    some_user: komodorpcutil::KomodoRPC, 
    pub_key: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("setpubkey");
    
    let method_body: String = String::from("[\"")
                    + &pub_key.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn set_tx_fee(
    some_user: komodorpcutil::KomodoRPC, 
    amount: f64)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("settxfee");
    
    let method_body: String = String::from("[")
                    + &amount.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn sign_message(
    some_user: komodorpcutil::KomodoRPC, 
    address: String,
    message: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("settxfee");
    
    let method_body: String = String::from("[\"")
                    + &address.to_string()
                    + &String::from("\",\"")
                    + &message.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


// TODO: recheck, no examples
pub fn wallet_lock(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("walletlock");
    
    let method_body: String = String::from("[]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


// no example provided and no default provided for timeout
pub fn wallet_pass_phrase(
    some_user: komodorpcutil::KomodoRPC, 
    pass_phrase: String,
    timeout: Option<f64>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("walletpassphrase");
    
    let method_body: String = String::from("[\"")
                    + &pass_phrase.to_string()
                    + &String::from("\",")
                    + &timeout.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


// 
pub fn wallet_pass_phrase_change(
    some_user: komodorpcutil::KomodoRPC, 
    old_pass_phrase: String,
    new_pass_phrase: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("walletpassphrasechange");
    
    let method_body: String = String::from("[\"")
                    + &old_pass_phrase.to_string()
                    + &String::from("\",\"")
                    + &new_pass_phrase.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn z_export_key(
    some_user: komodorpcutil::KomodoRPC, 
    z_address: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_exportkey");
    
    let method_body: String = String::from("[\"")
                    + &z_address.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn z_export_viewing_key(
    some_user: komodorpcutil::KomodoRPC, 
    z_address: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_exportviewingkey");
    
    let method_body: String = String::from("[\"")
                    + &z_address.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn z_export_wallet(
    some_user: komodorpcutil::KomodoRPC, 
    file_name: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_exportwallet");
    
    let method_body: String = String::from("[\"")
                    + &file_name.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn z_get_balance(
    some_user: komodorpcutil::KomodoRPC, 
    address: String,
    minconf: Option<u32>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_getbalance");
    let temp_minconf: String = minconf.unwrap_or(1);
    let method_body: String = String::from("[\"")
                    + &file_name.to_string()
                    + &String::from("\",")
                    + &temp_minconf.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn z_get_new_address(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_getnewaddress");
    
    let method_body: String = String::from("[]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn z_get_operation_result(
    some_user: komodorpcutil::KomodoRPC,
    operation_id: Option<String>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_getoperationresult");
    
    let temp_operation_id: String = operation_id.unwrap_or("".to_string());
    let method_body: String = String::from("[[\"")
                    + &temp_operation_id.to_string()
                    + &String::from("\"]]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


// TODO: arrays as argument?
pub fn z_get_operation_status(
    some_user: komodorpcutil::KomodoRPC,
    operation_id: Option<String>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_getoperationstatus");
    
    let temp_operation_id: String = operation_id.unwrap_or("".to_string());
    let method_body: String = String::from("[[\"")
                    + &temp_operation_id.to_string()
                    + &String::from("\"]]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}



pub fn z_get_total_balance(
    some_user: komodorpcutil::KomodoRPC,
    minconf: Option<u32>,
    include_watch_only: Option<bool>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_gettotalbalance");
    let temp_minconf = minconf.unwrap_or(1);
    let temp_include_watch_only = include_watch_only.unwrap_or(false);
    let method_body: String = String::from("[")
                    + &temp_minconf.to_string()
                    + &String::from(",")
                    + &temp_include_watch_only.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn z_import_key(
    some_user: komodorpcutil::KomodoRPC,
    z_private_key: String,
    rescan: Option<String>,
    start_height: Option<u32>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_importkey");
    let temp_rescan = rescan.unwrap_or("whenkeyisnew".to_string());
    let temp_start_height = start_height.unwrap_or(0);
    let method_body: String = String::from("[\"")
                    + &z_private_key.to_string()
                    + &String::from("\",\"")
                    + &temp_rescan.to_string()
                    + &String::from("\",")
                    + &temp_start_height.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}



pub fn z_import_viewing_key(
    some_user: komodorpcutil::KomodoRPC,
    z_private_key: String,
    rescan: Option<String>,
    start_height: Option<u32>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_importviewingkey");
    let temp_rescan = rescan.unwrap_or("whenkeyisnew".to_string());
    let temp_start_height = start_height.unwrap_or(0);
    let method_body: String = String::from("[\"")
                    + &z_private_key.to_string()
                    + &String::from("\",\"")
                    + &temp_rescan.to_string()
                    + &String::from("\",")
                    + &temp_start_height.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}



pub fn z_import_wallet(
    some_user: komodorpcutil::KomodoRPC,
    file_name: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_importwallet");
    
    let method_body: String = String::from("[\"")
                    + &file_name.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}


pub fn z_list_addresses(
    some_user: komodorpcutil::KomodoRPC
    include_watch_only: Option<bool>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_listaddresses");
    let temp_include_watch_only = include_watch_only.unwrap_or(false);
    let method_body: String = String::from("[")
                    + &include_watch_only.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}



// TODO: handle if empty case
pub fn z_list_operation_ids(
    some_user: komodorpcutil::KomodoRPC
    status: Option<String>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_listoperationids");
    let temp_include_watch_only = include_watch_only.unwrap_or("".to_string());
    let method_body: String = String::from("[")
                    + &include_watch_only.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}





// WARNING: UNFINISHED METHOD
pub fn z_listreceivedbyaddress(
    some_user: komodorpcutil::KomodoRPC
    include_watch_only: Option<bool>)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("z_listaddresses");
    let temp_include_watch_only = include_watch_only.unwrap_or(false);
    let method_body: String = String::from("[")
                    + &include_watch_only.to_string()
                    + &String::from("]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}