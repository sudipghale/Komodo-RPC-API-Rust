#![warn(missing_docs)]
//!
//! This is the documentation for 'Address' module of Komodo.
//!
//! The 'Address' module of Komodo contains functionality of the 'Address' noted on the
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
//! [Komodo website]: https://docs.komodoplatform.com/basic-docs/smart-chains/smart-chain-api/address.html
//!

use super::komodorpcutil;
use komodorpcutil::KomodoRPC;

/// The getaddressbalance method returns the confirmed balance for an address, or addresses. It requires addressindex to be enabled.
///
/// # Arguments
/// * `some_user` - A komodorpcutil::KomodoRPC object that holds the USER and RPC information.
/// * `address` - A required string of the address
///
///
/// # Response
///
/// * `satoshis` -  (number)	the difference in satoshis
/// * `txid`     -  (string)	the related transaction id
/// * `index`    -  (number)	the related input or output index
/// * `height`	 -  (number)	the block height
/// * `address`	 -  (string)	the address
///
/// # Examples
/// ```
/// TODO: Update examples
/// let result = komodo::cross_chain::create_multisig(some_user, address);  
/// ```
///

pub fn get_address_balance(
    some_user: komodorpcutil::KomodoRPC,
    v_address: Vec<String>,
) -> Result<(), reqwest::Error> {
    // implement for vec of addrs

    let mut addr_list = String::from("[");

    for addr in &v_address {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing the address to string
    }

    if (v_address.len() > 0)
    // >1
    {
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string(); // appending upto the last one ,
    }

    addr_list = addr_list + &"]"; //adding the last address]

    let params = String::from("[{\"addresses\": ") + &addr_list + "}]";
    //let params = format!("[{\"addresses\": {0}}]", addr_list); //need to fix to use format

    //BODY HAS TO BE
    let method_name: String = String::from("getaddressbalance");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// The getaddressdeltas method returns all confirmed balance changes of an address.
/// The user can optionally limit the response to a given interval of blocks.
/// The method requires addressindex to be enabled.
///
/// # Arguments
/// * `some_user` -  A komodorpcutil::KomodoRPC object that holds the USER and RPC information.
/// * `address`   -	 (string)	the address
/// * `start`  	  -  (number)	the start block height
/// * `end`	      -  (number)	the end block height
/// * `chainInfo` -  (boolean)	include chain info in results (only applies if start and end specified)
///
///
/// # Response
/// * `satoshis`  -  (number)	the difference in satoshis
/// * `txid`	  -  (string)	the related transaction id
/// * `index`	  -  (number)	the related input or output index
/// * `height`	  -  (number)	the block height
/// * `address`	  -  (string)	the address
/// %%%

pub fn get_address_deltas(
    some_user: komodorpcutil::KomodoRPC,
    v_address: Vec<String>,
    start: u32,
    end: u32,
    chainInfo: bool,
) -> Result<(), reqwest::Error> {
    let mut addr_list = String::from("[");

    for addr in &v_address {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
    }

    if (v_address.len() > 0)
    // >1
    {
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string();
    }

    addr_list = addr_list + &"]";

    let params = String::from("[{\"addresses\": ")
        + &addr_list
        + ",\"start\":"
        + &start.to_string()
        + ",\"end\":"
        + &end.to_string()
        + ",\"chainInfo\":"
        + &chainInfo.to_string()
        + "}]";
    let method_name: String = String::from("getaddressdeltas");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// The getaddressdeltas method returns all confirmed balance changes of an address.
/// The user can optionally limit the response to a given interval of blocks.
/// The method requires addressindex to be enabled.
///
/// # Arguments
/// * `some_user` -  A komodorpcutil::KomodoRPC object that holds the USER and RPC information.
/// * `address`   -	 (string)	the address
///
///
/// # Response
/// * `txid`      -  (string)	the related txid
/// * `index`     -  (number)	the related input or output index
/// * `satoshis`  -  (number)	the difference in satoshis
/// * `timestamp` -  (number)	the time the transaction entered the mempool (seconds)
/// * `prevtxid`  -  (string)	the previous txid (if spending)
/// * `prevout`   -  (string)	the previous transaction output index (if spending)
/// %%%

pub fn get_address_mem_pool(
    some_user: komodorpcutil::KomodoRPC,
    v_address: Vec<String>,
) -> Result<(), reqwest::Error> {
    let mut addr_list = String::from("[");

    for addr in &v_address {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
    }

    if (v_address.len() > 0)
    // >1
    {
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string();
    }

    addr_list = addr_list + &"]";

    let params = String::from("[{\"addresses\": ") + &addr_list + "}]";
    let method_name: String = String::from("getaddressmempool");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///The getaddresstxids method returns the txids for an address, or addresses.
/// It requires addressindex to be enabled.
///
/// # Arguments
/// * `address`	  - (string)	the address
/// * `start`	  - (number)	the start block height
/// * `end`	      - (number)	the end block height

///
/// # Response
/// * `transaction_id` - (string)	the addresse
/// %%%

pub fn get_address_tx_ids(
    some_user: komodorpcutil::KomodoRPC,
    v_address: Vec<String>,
    start: u32,
    end: u32,
) -> Result<(), reqwest::Error> {
    let mut addr_list = String::from("[");

    for addr in &v_address {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
    }

    if (v_address.len() > 0)
    // >1
    {
        // why need to cut last substring
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string();
    }

    addr_list = addr_list + &"]"; //& for String -> &string

    let params = String::from("[{\"addresses\": ")
        + &addr_list
        + ",\"start\":"
        + &start.to_string()
        + ",\"end\":"
        + &end.to_string()
        + "}]";
    //let params = format!("[{\"addresses\": {0}}]", addr_list); //need to fix to use format

    let payload="{\"jsonrpc\": \"1.0\", \"id\":\"curltest\", \"method\": \"getaddresstxids\",\"params\": [{\"addresses\": [\"RG81GbnXb4rmASYuhgAdfUdFBVqyVbyhde\"],\"start\":1,\"end\":200}]}\n\n\n";

    let method_name: String = String::from("getaddresstxids");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    println!("the payload is{}", payload);
    println!("the body is{:?}", data);
    let result = komodorpcutil::request(some_user.clone(), data);
    return result;
}

/// The getaddressdeltas method returns all confirmed balance changes of an address.
/// The user can optionally limit the response to a given interval of blocks.
/// The method requires addressindex to be enabled.
///
/// # Arguments
/// * `some_user` -  A komodorpcutil::KomodoRPC object that holds the USER and RPC information.
/// * `address`   -	 (string)	the address
/// * `chainInfo`-	(boolean)	include chain info with results
/// # Response
/// * `address`-	(string)	the address
/// * `txid`-	(string)	the output txid
/// * `height`-	(number)	the block height
/// * `outputIndex`-	(number)	the output index
/// * `script`-	(string)	the script hex encoded
/// * `satoshis`-	(number)	the number of satoshis of the output
/// %%%
pub fn get_address_utxos(
    some_user: komodorpcutil::KomodoRPC,
    v_address: Vec<String>,
    chainInfo: bool,
) -> Result<(), reqwest::Error> {
    let mut addr_list = String::from("[");

    for addr in &v_address {
        addr_list = addr_list + "\"" + addr + "\"" + &",";
    }

    if (v_address.len() > 0)
    // >1
    {
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string();
    }

    addr_list = addr_list + &"]";

    let params = String::from("[{\"addresses\": ")
        + &addr_list
        + ",\"chainInfo\":"
        + &chainInfo.to_string()
        + "}]";

    let method_name: String = String::from("getaddressutxos");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

///The getsnapshot method returns a snapshot of addresses and
/// their amounts at the Smart Chain's current height.
/// The method requires addressindex to be enabled.
///
/// # Arguments
/// * `top`	- (number, optional)	Only return this many addresses, i.e. top N rich list
///
/// # Response
/// * `addresses` -	(array of jsons)	the array containing the address and amount details
/// * `addr` -	(string)	an address
/// * `amount` -	(number)	the amount of coins in the above address
/// * `total` -	(numeric)	the total amount in snapshot
/// * `average` -	(numeric)	the average amount in each address
/// * `utxos` -	(number)	the total number of utxos in snapshot
/// * `total_addresses` -	(number)	the total number of addresses in snapshot,
/// * `start_height` -	(number)	the block height snapshot began
/// * `ending_height` -	(number)	the block height snapshot finished,
/// * `start_time` -	(number)	the unix epoch time snapshot started
/// * `end_time` -	(number)	the unix epoch time snapshot finished
/// %%%
pub fn get_snapshot(
    some_user: komodorpcutil::KomodoRPC,
    top: Option<u32>,
) -> Result<(), reqwest::Error> {
    let method_body: String;
    let temp_top = top.unwrap_or(0);
    if (temp_top > 0) {
        method_body = String::from("[\"") + &temp_top.to_string() + "\"]";
    } else {
        method_body = String::from("[]");
    }

    let method_name: String = String::from("getsnapshot");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}
