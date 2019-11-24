#![warn(missing_docs)]
//! 
//! This is the documentation for 'Util' module of Komodo.
//!
//! The 'Util' module of Komodo contains functionality of the 'Util' noted on the
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
//! [Komodo website]: https://docs.komodoplatform.com/basic-docs/smart-chains/smart-chain-api/util.html
//! 


// TODO: - run fmt and clippy
//       - document all methods
//          - more advanced examples
//       - ? potential fix for no function/method overloading if extra time
//              -> trait object static/dynamic dispatch for multi-type parameter
//       - ? some_user parameter may be simplified further


use super:: komodorpcutil;
use komodorpcutil::KomodoRPC;

/// The create_multisig method creates a multi-signature address with n signature(s) of m key(s) required. The method returns a json object with the address and redeemScript.
/// 
/// # Arguments
/// 
/// * `number_required` - A required u32 that represents the number of required signatures out of the n key(s) or address(es).
/// * `keys` - A required string that represents a json array of keys which are addresses or hex-encoded public keys.
/// 
/// # Response
/// 
/// * `address` - The value of the new multisig address.
/// * `redeemScript` - The string value of the hex-encoded redemption script.
/// 
/// # Examples
/// ```
/// TODO: Update examples
/// let result = komodo::cross_chain::create_multisig(some_user, 
///                                                                    "string".to_string(),
///                                                                    number_u32, 
///                                                                    None);
/// 
/// ```
/// 
pub fn create_multisig(
    some_user: komodorpcutil::KomodoRPC, 
    number_required: u32,
    keys: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("createmultisig");
    
    let method_body: String = String::from("[\"")
                    + &number_required.to_string()
                    + &String::from("\",[\"")
                    + &keys.to_string()
                    + &String::from("\"]]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}

pub fn decode_ccopret(
    some_user: komodorpcutil::KomodoRPC,
    script_pub_key: String)
    ->Result<(), reqwest::Error>
{
    
    let method_name: String = String::from("decodeccopret");
    
    let method_body: String = String::from("[\"")
                    + &script_pub_key.to_string()
                    + &String::from("\"]");
    
    let data: String = String::from(komodorpcutil::generate_body(some_user.clone(), method_name, method_body));
    let result = komodorpcutil::request(some_user.clone(), data);
    
    return result;
    
}



pub fn estimate_fee(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    unimplemented!();
    
}

pub fn estimate_priority(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    unimplemented!();
    
}

pub fn invalidate_block(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    unimplemented!();
    
}

pub fn reconsider_block(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    unimplemented!();
    
}

pub fn tx_notarized_confirmed(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    unimplemented!();
    
}

pub fn validate_address(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    unimplemented!();
    
}

pub fn verify_message(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    unimplemented!();
    
}

pub fn z_validate_address(
    some_user: komodorpcutil::KomodoRPC)
    ->Result<(), reqwest::Error>
{
    
    unimplemented!();
    
}
