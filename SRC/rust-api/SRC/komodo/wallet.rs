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
//addmultisigaddress has been DEPRECATED

/*
backupwallet
backupwallet "destination"
The backupwallet method safely copies the wallet.dat file to the indicated destination. The destination input accepts only alphanumeric characters.
This method requires that the coin daemon have the exportdir runtime parameter enabled.
#Arguments
Name	        Type                       	Description
"destination"	(string, required)	the destination filename, saved in the directory set by the exportdir runtime parameter
#Response
Name	   Type	     Description
"path"	(string)	the full path of the destination file
*/
pub fn backup_wallet(
    someUser: komodorpcutil::KomodoRPC,
    destination: String,
) -> Result<(), reqwest::Error> {
    let method_body = String::from("[\"") + &destination + "\"]"; // String::from(format!("[/"{0}/"]",temp_top));
    let method_name: String = String::from("backupwallet");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    println!("the body is{:?}", data);
    komodorpcutil::request(someUser.clone(), data)
}

/*
dumpprivkey
dumpprivkey "address"
The dumpprivkey method reveals the private key corresponding to the indicated address.
See also importprivkey.
#Arguments
Name	    Type               	Description
"address"	(string, required)	the address for the private key
#Response
Name   	Type     	Description
"data"	(string)	the private key
*/
pub fn dump_priv_key(
    someUser: komodorpcutil::KomodoRPC,
    address: String,
) -> Result<(), reqwest::Error> {
    let method_body = String::from("[\"") + &address + "\"]";
    let method_name: String = String::from("dumpprivkey");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    println!("the body is{:?}", data);
    komodorpcutil::request(someUser.clone(), data)
}

/*
dumpwallet
dumpwallet "filename"
The dumpwallet method dumps all transparent-address wallet keys into a file, using a human-readable format.
Overwriting an existing file is not permitted. The destination parameter accepts only alphanumeric characters.
This method requires that the coin daemon have the exportdir runtime parameter enabled.
#Arguments
Name      	Type	              Description
"filename"	(string, required)	the filename, saved in the folder set by the exportdir runtime parameter
Response
Name	  Type    	Description
"path"	(string)	the full path of the destination file
*/

pub fn dump_wallet(
    someUser: komodorpcutil::KomodoRPC,
    filename: String,
) -> Result<(), reqwest::Error> {
    let method_body = String::from("[\"") + &filename + "\"]";
    let method_name: String = String::from("dumpwallet");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    println!("the body is{:?}", data);
    komodorpcutil::request(someUser.clone(), data)
}

/*
encryptwallet
encryptwallet "passphrase"
TO DO???? Using the encryptwallet method will shutdown the Komodo daemon (komodod).
This feature is available only on chains where -ac_public is enabled. Chains that feature private transactions cannot use this feature.
The encryptwallet method encrypts the wallet with the indicated passphrase.
For more information, please see these instructions: Encrypt Komodo's wallet.dat File
This method is for first-time encryption only. After the first encryption, any calls that interact with private keys will require the passphrase via walletpassphrase prior to calling the corresponding method. This includes methods that create a transaction, dump a private key for an address, sign a transaction, etc.
#Arguments
Name      	Type    	Description
passphrase	(string)	the passphrase for wallet encryption; the passphrase must be at least 1 character, but should be many
#Response
Text Response
wallet encrypted; Komodo server stopping, restart to run with encrypted wallet. The keypool has been flushed, you need to make a new backup
*/
pub fn encrypt_wallet(
    someUser: komodorpcutil::KomodoRPC,
    passphrase: String,
) -> Result<(), reqwest::Error> {
    let method_body = String::from("[\"") + &passphrase + "\"]";
    let method_name: String = String::from("encruptwallet");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    println!("the body is{:?}", data);
    komodorpcutil::request(someUser.clone(), data)
}
/*
 * getaccount
getaccount "address"
The getaccount method returns the account associated with the given address.
#Arguments
Name	    Type	              Description
"address"	(string, required)	the address
#Response
Name        	Type     	Description
"accountname"	(string)	the account address
 *
 */

pub fn get_account(
    someUser: komodorpcutil::KomodoRPC,
    address: String,
) -> Result<(), reqwest::Error> {
    let method_body = String::from("[\"") + &address + "\"]";
    let method_name: String = String::from("getaccount");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    println!("the body is{:?}", data);
    komodorpcutil::request(someUser.clone(), data)
}

/*
getbalance
getbalance ( "account" minconf includeWatchonly )
The getbalance method returns the server's total available balance.
The account input is deprecated.
#Arguments
Name	     Type             	Description
"account"	(string, optional)	DEPRECATED if provided, it MUST be set to the empty string "" or to the string "*"
minconf	(numeric, optional, default=1)	only include transactions confirmed at least this many times
includeWatchonly	(bool, optional, default=false)	also include balance in watchonly addresses (see importaddress)
#Response
Name	Type	Description
amount	(numeric)	the total amount
*/
pub fn get_balance(
    //TODO check def value and if conditions
    SomeUser: komodorpcutil::KomodoRPC,
    minconf: Option<u32>,
    includeWatchonly: Option<bool>,
) -> Result<(), reqwest::Error> {
    let method_body: String;
    let temp_minconf = minconf.unwrap_or(1); //Default value is 1
    let temp_includeWatchonly: String = includeWatchonly.unwrap_or(false).to_string();
    if (!(temp_minconf == 1) && !(temp_includeWatchonly.is_empty())) {
        method_body = String::from("[")
            + &temp_minconf.to_string()
            + &",".to_string()
            + &temp_includeWatchonly
            + &String::from("]");
    } else if ((temp_minconf == 1) && !(temp_includeWatchonly.is_empty())) {
        method_body = String::from("[") + &temp_minconf.to_string() + &String::from("]");
    } else if (!(temp_minconf == 1) && (temp_includeWatchonly.is_empty())) {
        method_body = String::from("[") + &temp_includeWatchonly.to_string() + &String::from("]");
    } else
    //  ((temp_bantime==0) && !(temp_absolute.is_empty()))
    {
        method_body = String::from("[") + &String::from("]");
    }

    let method_name: String = String::from("getbalance");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/*
getbalance64
This method is part of the new ac_staked functionality.// DIDNT IMPLEMENTED NOT ENOUGH INFO....
The getbalance64 method is used only on Smart Chains that are utilizing the ac_staked functionality.
 On KMD-based Proof-of-Stake (PoS) Smart Chains, all staked coins are placed into one of 64 segments (segid's').
 The getbalance64 method returns the balance of coins in each segid.
*/

/*
getnewaddress
getnewaddress ( "account" )
The getnewaddress method returns a new address for receiving payments.
#Arguments
Name	Type	Description
"account"	(string, optional)	DEPRECATED: If provided, the account MUST be set to the empty string "" to represent the default account; passing any other string will result in an error
#Response
Name	Type	Description
"address"	(string)	the new address
*/
pub fn get_new_address(SomeUser: komodorpcutil::KomodoRPC) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("getnewaddress");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

/*
getrawchangeaddress
The getrawchangeaddress returns a new address that can be used to receive change.
This is for use with raw transactions, NOT normal use.
#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
"address"	(string)	the address
*/
pub fn get_raw_change_address(SomeUser: komodorpcutil::KomodoRPC) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("getrawchangeaddress");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

/*
getreceivedbyaddress
getreceivedbyaddress "address" ( minconf )
The getreceivedbyaddress method returns the total amount received by the given address in transactions with at least minconf confirmations.
#Arguments
Name    	Type	Description
"address"	(string, required)	the address for transactions
minconf  	(numeric, optional, default=1)	only include transactions confirmed at least this many times
#Response
Name   	Type     	Description
amount	(numeric)	the total amount of the relevant coin received at this address
*/

pub fn get_receive_by_address(
    SomeUser: komodorpcutil::KomodoRPC,
    address: String,
    min_conf: Option<u32>,
) -> Result<(), reqwest::Error> {
    let method_body: String;
    let temp_min_conf: String = min_conf.unwrap_or(1).to_string(); // default 1///TO DO
    if (temp_min_conf.is_empty()) {
        method_body =
            String::from("[\"") + &address.to_string() + &String::from("\"") + &String::from("]");
    } else
    //if (!temp_gen_proc_limit.is_empty())
    {
        method_body = String::from("[\"")
            + &address.to_string()
            + &"\",".to_string()
            + &temp_min_conf
            + &String::from("]");
    }

    let payload =  "{\"jsonrpc\": \"1.0\", \"id\":\"curltest\",  \"method\": \"getreceivedbyaddress\", \"params\": [\"RJSDZjp7kjBNhHsbECDE1jwYNK7af41pZN\", 6] }\n";
    let method_name: String = String::from("getreceivedbyaddress");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    println!("payload is {:?}", payload);
    println!("the body is{:?}", data);
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

/*
gettransaction
gettransaction "txid" ( includeWatchonly )
The gettransaction method queries detailed information about transaction txid. This command applies only to txid's that are in the user's local wallet.
#Arguments
Name	Type	Description
"txid"	(string, required)	the transaction id
"includeWatchonly"	(bool, optional, default=false)	whether to include watchonly addresses in the returned balance calculation and in the details[] returned values
#Response
Name	Type	Description
"amount"	(numeric)	the transaction amount
"confirmations"	(numeric)	a confirmation number that is aware of the dPoW security service
"rawconfirmations"	(numeric)	the raw confirmations (number of blocks on top of this transaction's block)
"blockhash"	(string)	the block hash
"blockindex"	(numeric)	the block index
"blocktime"	(numeric)	the time in seconds since epoch (1 Jan 1970 GMT)
"txid"	(string)	the transaction id
"time"	(numeric)	the transaction time in seconds since epoch (1 Jan 1970 GMT)
"timereceived"	(numeric)	the time received in seconds since epoch (1 Jan 1970 GMT)
"details" : [ ... ]	(array)
"account"	(string)	DEPRECATED the account name involved in the transaction; can be "" for the default account
"address"	(string)	the address involved in the transaction
"category"	(string)	the category - either send or receive
"amount"	(numeric)	the amount
"vout"	(numeric)	the vout value
"vjoinsplit" : [ ... ]	(array of json objects)
"anchor"	(string)	merkle root of note commitment tree
"nullifiers" : [ ... ]	(array of strings)
"hex"	(string)
"commitments" : [ ... ]	(array of strings)
"hex"	(string)
"macs" : [ ... ]	(array of strings)
"hex"	(string)
"vpub_old"	(numeric)	the amount removed from the transparent value pool
"vpub_new"	(numeric)	the amount added to the transparent value pool
"hex"	(string)	transaction data translated into hex
*/
pub fn get_transaction(
    SomeUser: komodorpcutil::KomodoRPC,
    tx_id: String,
    include_watch_only: Option<bool>,
) -> Result<(), reqwest::Error> {
    let method_body: String;
    let temp_include_watch_only: String = include_watch_only.unwrap_or(false).to_string();
    method_body = String::from("[\"")
        + &tx_id.to_string()
        + &String::from("\",")
        + &temp_include_watch_only
        + &String::from("]");
    let method_name: String = String::from("gettransaction");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    println!("the body is{:?}", data);
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

/*
getunconfirmedbalance
getunconfirmedbalance
The getunconfirmedbalance method returns the server's total unconfirmed balance.
#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
(none)
*/
pub fn get_unconfirmed_balance(SomeUser: komodorpcutil::KomodoRPC) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("getunconfirmedbalance");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

/*
getwalletinfo
The getwalletinfo method returns an object containing various information about the wallet state.
#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
"walletversion"	(numeric)	the wallet version
"balance"	(numeric)	the total confirmed balance of the wallet
"unconfirmed_balance"	(numeric)	the total unconfirmed balance of the wallet
"immature_balance"	(numeric)	the total immature balance of the wallet
"txcount"	(numeric)	the total number of transactions in the wallet
"keypoololdest"	(numeric)	the timestamp (seconds since GMT epoch) of the oldest pre-generated key in the key pool
"keypoolsize"	(numeric)	how many new keys are pre-generated
"unlocked_until"	(numeric)	the timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is unlocked for transfers, or 0 if the wallet is locked
"paytxfee"	(numeric)	the transaction fee configuration, given as the relevant COIN per KB
*/
pub fn get_wallet_info(SomeUser: komodorpcutil::KomodoRPC) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("getwalletinfo");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

/*
importaddress
importaddress "address" ( "label" rescan )
The importaddress method adds an address or script (in hex) that can be watched as if it were in your wallet, although it cannot be used to spend.
This call can take an increased amount of time to complete if rescan is true.
#Arguments
Name	Type	Description
"address"	(string, required)	the address to watch
"label"	(string, optional, default="")	an optional label
rescan	(boolean, optional, default=true)	rescan the wallet for transactions
#Response
Name	Type	Description
(none)
*/
pub fn import_address(
    //TODO check def value and if conditions
    SomeUser: komodorpcutil::KomodoRPC,
    address: String,
    label: Option<String>,
    rescan: Option<bool>,
) -> Result<(), reqwest::Error> {
    let method_body: String;
    let temp_label: String = label.unwrap_or("".to_string()).to_string();
    let temp_rescan: String = rescan.unwrap_or(true).to_string();

    if (temp_label.is_empty()) {
        method_body =
            String::from("[\"") + &address + &"\",".to_string() + &temp_rescan + &String::from("]");
    } else {
        method_body = String::from("[\"")
            + &address
            + &"\",\"".to_string()
            + &temp_label
            + &"\",".to_string()
            + &temp_rescan
            + &String::from("]");
    }
    let method_name: String = String::from("importaddress");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}
/*
importprivkey
importkey "komodoprivkey" ( "label" rescan )
The importprivkey method adds a private key to your wallet.
This call can take minutes to complete if rescan is true.
See also dumpprivkey.
#Arguments
Name	Type	Description
"privkey"	(string, required)	the private key (see dumpprivkey)
"label"	(string, optional, default="")	an optional label
rescan	(boolean, optional, default=true)	rescan the wallet for transactions
Response
Name	Type	Description
addresses	(string)	the public address
*/
pub fn import_priv_key(
    //TODO check def value and if conditions
    SomeUser: komodorpcutil::KomodoRPC,
    priv_key: String,
    label: Option<String>,
    rescan: Option<bool>,
) -> Result<(), reqwest::Error> {
    let method_body: String;
    let temp_label: String = label.unwrap_or("".to_string()).to_string();
    let temp_rescan: String = rescan.unwrap_or(true).to_string();

    if (temp_label.is_empty()) {
        method_body = String::from("[\"")
            + &priv_key
            + &"\",".to_string()
            + &temp_rescan
            + &String::from("]");
    } else {
        method_body = String::from("[\"")
            + &priv_key
            + &"\",\"".to_string()
            + &temp_label
            + &"\",".to_string()
            + &temp_rescan
            + &String::from("]");
    }
    let method_name: String = String::from("importprivkey");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    println!("the body is{:?}", data);

    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}
/*
importwallet
importwallet "filename"
The importwallet method imports transparent-address keys from a wallet-dump file (see dumpwallet).
#Arguments
Name	Type	Description
"filename"	(string, required)	the wallet file
#Response
Name	Type	Description
(none)	*/

pub fn import_wallet(
    someUser: komodorpcutil::KomodoRPC,
    file_name: String,
) -> Result<(), reqwest::Error> {
    let params = String::from("[\"") + &file_name + "\"]";

    let method_name: String = String::from("importwallet");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    println!("the body is{:?}", data);
    let result = komodorpcutil::request(someUser.clone(), data);
    return result;
}

/**
 * keypoolrefill
keypoolrefill ( newsize )
The keypoolrefill method refills the keypool.
#Arguments
Name	Type	Description
newsize	(numeric, optional, default=100)	the new keypool size
#Response
Name	Type	Description
(none)
 *
 */

pub fn key_pool_refill(
    SomeUser: komodorpcutil::KomodoRPC,
    new_size: Option<u32>,
) -> Result<(), reqwest::Error> {
    let method_body: String;
    let temp_new_size: String = new_size.unwrap_or(100).to_string();
    method_body = String::from("[") + &temp_new_size + &String::from("]");
    let method_name: String = String::from("keypoolrefill");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    println!("the body is{:?}", data);
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

/**
 * listaddressgroupings
listaddressgroupings
The listaddressgroupings method lists groups of addresses which have had their common ownership made public by common use as inputs or as the resulting change in past transactions.
#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
"address",	(string)	the address
amount,	(numeric)	the amount
"account"	(string, optional)	(DEPRECATED) the account
 *
 */
pub fn list_address_groupings(SomeUser: komodorpcutil::KomodoRPC) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("listaddressgroupings");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/**
 * listlockunspent
listlockunspent
The listlockunspent method returns a list of temporarily non-spendable outputs.
See the lockunspent call to lock and unlock transactions for spending.
#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
"txid"	(string)	the transaction id locked
"vout"	(numeric)	the vout value
 */
pub fn list_lock_unspent(SomeUser: komodorpcutil::KomodoRPC) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("listlockunspent");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/**
 * listreceivedbyaddress
listreceivedbyaddress ( minconf includeempty includeWatchonly)
The listreceivedbyaddress method lists balances by receiving address.
#Arguments
Name	Type	Description
minconf	(numeric, optional, default=1)	the minimum number of confirmations before payments are included
includeempty	(numeric, optional, default=false)	whether to include addresses that haven't received any payments// TO DO must be bool
includeWatchonly	(bool, optional, default=false)	whether to include watchonly addresses (see 'importaddress')
#Response
Name	Type	Description
"involvesWatchonly"	(bool)	only returned if imported addresses were involved in transaction
"address"	(string)	the receiving address
"account"	(string)	DEPRECATED the account of the receiving address; the default account is ""
"amount"	(numeric)	the total amount received by the address
"confirmations"	(numeric)	a confirmation number that is aware of the dPoW security service
"rawconfirmations"	(numeric)	the raw confirmations of the most recent transaction included (number of blocks on top of this transaction's block
 */

pub fn list_received_by_address(
    SomeUser: komodorpcutil::KomodoRPC,
    min_conf: Option<u32>,
    include_empty: Option<bool>,
    include_watch_only: Option<bool>,
) -> Result<(), reqwest::Error> {
    let method_body: String;
    let temp_min_conf: String = min_conf.unwrap_or(1).to_string();
    let temp_include_empty: String = include_empty.unwrap_or(false).to_string();
    let temp_include_watch_only: String = include_watch_only.unwrap_or(false).to_string();

    method_body = String::from("[")
        + &temp_min_conf
        + &String::from(",")
        + &temp_include_empty
        + &String::from(",")
        + &temp_include_watch_only
        + &String::from("]");
    let method_name: String = String::from("listreceivedbyaddress");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    println!("the body is{:?}", data);
    komodorpcutil::request(SomeUser.clone(), data)
}

/** TODO COMPLETE
 * listsinceblock
listsinceblock ( "blockhash" target-confirmations includeWatchonly )
The listsinceblock method queries all transactions in blocks since block blockhash, or all transactions if blockhash is omitted.
#Arguments
Name      	Type              	Description
"blockhash"	(string, optional)	the block hash from which to list transactions
target-confirmations	(numeric, optional)	the confirmations required (must be 1 or more)
includeWatchonly	(bool, optional, default=false)	include transactions to watchonly addresses (see also 'importaddress')
#Response
Name	Type	Description
"transactions":
"account"	(string)	DEPRECATED the account name associated with the transaction; will be "" for the default account
"address"	(string)	the address of the transaction (not present for move transactions -- category = move)
"category"	(string)	the transaction category; send has negative amounts, receive has positive amounts
"amount"	(numeric)	the amount of the relevant currency -- negative for the send category, and for the move category for moves outbound. It is positive for the receive category, and for the move category for inbound funds.
"vout"	(numeric)	the vout value
"fee"	(numeric)	the amount of the fee; this value is negative and only available for the send category of transactions
"confirmations"	(numeric)	a confirmation number that is aware of the dPoW security service
"rawconfirmations"	(numeric)	the raw confirmations of the transaction; available for send and receive category of transactions (number of blocks on top of this transaction's block)
"blockhash"	(string)	the block hash containing the transaction; available for the send and receive categories of transactions
"blockindex"	(numeric)	the block index containing the transaction; available for the send and receive categories of transactions
"blocktime"	(numeric)	the block time in seconds since epoch (1 Jan 1970 GMT)
"txid"	(string)	the transaction id; available for send and receive categories of transactions
"time"	(numeric)	the transaction time in seconds since epoch (Jan 1 1970 GMT)
"timereceived"	(numeric)	the time received in seconds since epoch (Jan 1 1970 GMT); available for send and receive category of transactions
"comment"	(string)	whether a comment is associated with the transaction
"to"	(string)	whether a 'to' comment is associated with the transaction
"lastblock"	(string)	the hash of the last block
 */
pub fn list_since_block(
    some_user: komodorpcutil::KomodoRPC,
    block_hash: Option<String>,
    target_conformations: Option<u32>,
    include_watch_only: Option<bool>,
) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("listsinceblock");
    let temp_block_hash: String = block_hash.unwrap_or("".to_string());
    let temp_target_conformations = target_conformations.unwrap_or(1);
    let temp_watch_only = include_watch_only.unwrap_or(false);

    let method_body: String = String::from("[\"")
        + &temp_block_hash
        + &String::from("\",")
        + &temp_target_conformations.to_string()
        + &String::from(",")
        + &temp_watch_only.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/**
 * TODO
 * listtransactions ( "account" count from includeWatchonly )
The listtransactions method returns up to count most recent transactions skipping the first from transactions for account.
#Arguments
Name            	Type                          	Description
"account"       	(string, optional)            	DEPRECATED the account name; should be "*"
count	            (numeric, optional, default=10)	the number of transactions to return
from	            (numeric, optional, default=0)	the number of transactions to skip
includeWatchonly	(bool, optional, default=false)	include transactions to watchonly addresses (see importaddress)
#Response
Name	Type	Description
"account"	(string)	DEPRECATED the account name associated with the transaction; it will be "" for the default account
"address"	(string)	the address of the transaction; not present for move transactions (category = move)
"category"	(string)	The transaction category. This property can be send
"amount"	(numeric)	The amount. This value is negative for the send category, and for the move category for moves outbound. It is positive for the receive category and for the move category for inbound funds.
"vout"	(numeric)	the vout value
"fee"	(numeric)	the amount of the fee; this is negative and only available for the send category of transactions
"confirmations"	(numeric)	a confirmation number that is aware of the dPoW security service
"rawconfirmations"	(numeric)	the raw confirmations of the transaction; available for send and receive category of transactions (number of blocks on top of this transaction's block)
"blockhash"	(string)	the block hash containing the transaction; available for the send and receive categories of transactions
"blockindex"	(numeric)	the block index containing the transaction; available for the send and receive categories of transactions
"txid"	(string)	the transaction id; available for the send and receive categories of transactions
"time"	(numeric)	the transaction time in seconds since epoch (midnight Jan 1 1970 GMT)
"timereceived"	(numeric)	the time received in seconds since epoch (midnight Jan 1 1970 GMT); available for the send and receive categories of transactions
"comment"	(string)	whether a comment is associated with the transaction
"otheraccount"	(string)	for the move category of transactions; indicates the account which sent the funds (for receiving funds, positive amounts), or went to (for sending funds, negative amounts)
"size"	(numeric)	transaction size in bytes
 *
 */

pub fn list_transactions(
    some_user: komodorpcutil::KomodoRPC,
    accoount: Option<String>,
    count: Option<u32>,
    from: Option<u32>,
    include_watch_only: Option<bool>,
) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("listtransactions");

    let method_body: String = String::from("[\"")
        + &"*".to_string()
        + &String::from("\",")
        + &count.unwrap_or(10).to_string()
        + &from.unwrap_or(0).to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/** TODO
  * listunspent ( minconf maxconf ["address", ... ] )
The listunspent method returns an array of unspent transaction outputs, with a range between minconf and maxconf (inclusive) confirmations. The method can, optionally, filter to only include txouts paid to specified addresses.
#Arguments
Name	Type	Description
minconf	(numeric, optional, default=1)	the minimum confirmations to filter
maxconf	(numeric, optional, default=9999999)	the maximum confirmations to filter
"address"	(string)	a series of addresses
#Response
Name	Type	Description
"txid"	(string)	the transaction id
"vout"	(numeric)	the vout value
"generated"	(boolean)	true if txout is a coinbase transaction output
"address"	(string)	the address
"account"	(string)	DEPRECATED the associated account, or "" for the default account
"scriptPubKey"	(string)	the script key
"amount"	(numeric)	the transaction amount
"confirmations"	(numeric)	a confirmation number that is aware of the dPoW security service
"rawconfirmations"	(numeric)	the raw confirmations (number of blocks on top of this transaction's block)
  */

/**
   * TODO
   * lockunspent unlock [{ "txid": "txid", "vout": n }, ... ]
The lockunspent method locks (unlock = false) or unlocks (unlock = true) specified transaction outputs. A locked transaction output will not be chosen by automatic coin selection, when spending the relevant coin. The locks are stored in memory only; at runtime a node always starts with zero locked outputs, and the locked output list is always cleared when a node stops or fails.
See the listunspent and listlockunspent calls to determine local transaction state and info.
#Arguments
Name	Type	Description
unlock	(boolean, required)	whether to unlock (true) or lock (false) the specified transactions
"txid"	(string)	the transaction id
"vout"	(numeric)	the output number
#Response
Name	Type	Description
true/false	(boolean)	whether the command was successful
   */

/*
  resendwallettransactions
The resendwallettransactions method immediately re-broadcasts unconfirmed wallet transactions to all peers. This method is intended only for testing; the wallet code periodically re-broadcasts automatically.
#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
"transaction_id"	(string)	an array of the rebroadcasted transaction id's
  */

pub fn resend_wallet_transactions(
    SomeUser: komodorpcutil::KomodoRPC,
) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("resendwallettransactions");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/** TODO
 * sendmany "account" { "address": amount, ... } ( minconf "comment" [ "address", ... ] )
The sendmany method can send multiple transactions at once. Amounts are double-precision floating point numbers.
#Arguments
Name	Type	Description
"account"	(string, required)	MUST be set to the empty string "" to represent the default account; passing any other string will result in an error
"amounts" { "address":amount, ... }	("string":numeric)	the address (string) and the value (double-precision floating numeric)
minconf	(numeric, optional, default=1)	only use the balance confirmed at least this many times
"comment"	(string, optional)	a comment
subtractfeefromamount	(string, optional)	a json array with addresses. The fee will be equally deducted from the amount of each selected address; the recipients will receive less than you enter in their corresponding amount field. If no addresses are specified here, the sender pays the fee.
"address"	(string)	subtract fee from this address
#Response
Name	Type	Description
"transaction_id"	(string)	the transaction id for the send; only 1 transaction is created regardless of the number of addresses
 */

/*
  * lockunspent unlock [{ "txid": "txid", "vout": n }, ... ]
The lockunspent method locks (unlock = false) or unlocks (unlock = true) specified transaction outputs. A locked transaction output will not be chosen by automatic coin selection, when spending the relevant coin. The locks are stored in memory only; at runtime a node always starts with zero locked outputs, and the locked output list is always cleared when a node stops or fails.
See the listunspent and listlockunspent calls to determine local transaction state and info.
#Arguments
Name	Type	Description
unlock	(boolean, required)	whether to unlock (true) or lock (false) the specified transactions
"txid"	(string)	the transaction id
"vout"	(numeric)	the output number
#Response
Name	Type	Description
true/false	(boolean)	whether the command was successful
  */

pub fn lock_unspent(
    someUser: komodorpcutil::KomodoRPC,
    unlock: bool,
    txid: String,
    vout: u32,
) -> Result<(), reqwest::Error> {
    let  payload = "{\"jsonrpc\": \"1.0\", \"id\":\"curltest\", \"method\": \"lockunspent\", \"params\": [false, [{\"txid\":\"d7ba45296c66e16eb61f27a4eef8848c7f5579fe801f277c1b0e074a4f47d6fd\",\"vout\":0}]] }";

    let method_body = String::from("[")
        + &unlock.to_string()
        + &(",[{\"txid\":\"").to_string()
        + &txid
        + &("\",\"vout\":").to_string()
        + &vout.to_string()
        + &("}]]").to_string();
    let method_name: String = String::from("lockunspent");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    println!("the payload is{:?}", payload);
    println!("the body is{:?}", data);
    komodorpcutil::request(someUser.clone(), data)
}


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


/*
Arguments
Name 	    Type 	                        Description
address 	(string) 	                    the private address.
minconf 	(numeric, optional, default=1) 	only include transactions confirmed at least this many times
#
Result
An array of json objects, each having the properties below.
Name 	Type 	                            Description
txid 	(string) 	                        the transaction id
amount 	(numeric) 	                        the amount of value in the note
memo 	(string) 	                        hexadecimal string representation of memo field
"confirmations" 	(numeric) 	            a confirmation number that is aware of the dPoW security service
"rawconfirmations" 	(numeric) 	            the raw confirmations (number of blocks on top of this transaction's block)
jsindex 	(sprout) 	(numeric, received only by sprout addresses) the joinsplit index
jsoutindex 	(numeric, received only by sprout addresses) 	the output index of the joinsplit
outindex 	(numeric, sapling) 	            the output index
change 	(boolean)                       	true if the address that received the note is also one of the sending addresses
*/

pub fn z_list_received_by_address(
    SomeUser: komodorpcutil::KomodoRPC,
    address: String,
    min_conf: Option<u32>,
) -> Result<(), reqwest::Error> {
    let method_body: String;
    method_body = String::from("[\"")
        + &address.to_string()
        + &String::from(" \",")
        + &min_conf.unwrap_or(1).to_string()
        + &String::from("]");
    let method_name: String = String::from("z_listreceivedbyaddress");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/*
TODO TEST THE SYSTEM
Function: z_listunspent
Arguments
Name 	Type 	Description
minconf 	(numeric, optional, default=1) 	the minimum confirmations to filter
maxconf 	(numeric, optional, default=9999999) 	the maximum confirmations to filter
includeWatchonly 	(bool, optional, default=false) 	whether to also include watchonly addresses (see z_importviewingkey)
addresses 	(array) 	a json array of z addresses (both Sprout and Sapling) to act as a filter; duplicate addresses are not allowed
address 	(string) 	a z address

#
Results

An array of json objects, each having the properties below.
Name 	Type 	Description
txid 	(string) 	the transaction id
jsindex 	(numeric) 	the joinsplit index
jsoutindex 	(numeric, only returned on sprout addresses) 	the output index of the joinsplit
outindex 	(numeric, only returned on sapling addresses) 	the output index
"confirmations" 	(numeric) 	a confirmation number that is aware of the dPoW security service
"rawconfirmations" 	(numeric) 	the raw confirmations (number of blocks on top of this transaction's block)
spendable 	(boolean) 	true if note can be spent by wallet, false if note has zero confirmations, false if address is watchonly
address 	(string) 	the shielded address
amount 	(numeric) 	the amount of value in the note
memo 	(string) 	hexadecimal string representation of memo field
change 	(boolean) 	true if the address that received the note is also one of the sending addresses
*/

pub fn z_list_unspent(
    SomeUser: komodorpcutil::KomodoRPC,
    min_conf: Option<u32>,
    max_conf: Option<u32>,
    include_watch_only: Option<bool>,
    addresses: Vec<String>,
    address: String,
) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("z_listunspent");
    let method_body: String;

    let mut addr_list = String::from("[");

    for addr in &addresses {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
    }
    if (addresses.len() > 0)
    // >1
    {
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string(); // cutting the last ,
    }
    addr_list = addr_list + &"]"; //& for String -> &string

    method_body = String::from("[")
        + &min_conf.unwrap_or(1).to_string()
        + &String::from(",")
        + &max_conf.unwrap_or(9999999).to_string()
        + &String::from(",")
        + &include_watch_only.unwrap_or(false).to_string()
        + &String::from(",")
        + &addr_list.to_string()
        + &String::from(",")
        + &address.to_string()
        + &String::from("]");

    komodorpcutil::request(
        SomeUser.clone(),
        komodorpcutil::generate_body(SomeUser.clone(), method_name, method_body),
    )
}

/*
TODO
                                        E X P E R I M E N T A L                    F E A T U R E
Check with Dr. Datta & Komodo Team
Function: z_mergetoaddress
Arguments
Name 	Type 	Description
fromaddresses 	(string, required)
"address" 	(string) 	can be a t address or a z address
"toaddress" 	(string, required) 	the t address or z address to receive the combined utxo
fee 	(numeric, optional, default=0.0001) 	the fee amount to attach to this transaction
transparent_limit 	(numeric, optional, default=50) 	limit on the maximum number of transparent utxos to merge; you may set this value to 0 to use the node option mempooltxinputlimit
shielded_limit 	(numeric, optional, default=10) 	limit on the maximum number of hidden notes to merge; you may set this value to 0 to merge as many as will fit in the transaction
"memo" 	(string, optional) 	encoded as hex; when toaddress is a z address, this value will be stored in the memo field of the new note
#
Response
Name 	Type 	Description
"remainingUTXOs" 	(numeric) 	the number of utxos still available for merging
"remainingTransparentValue" 	(numeric) 	the value of utxos still available for merging
"remainingNotes" 	(numeric) 	the number of notes still available for merging
"remainingShieldedValue" 	(numeric) 	the value of notes still available for merging
"mergingUTXOs" 	(numeric) 	the number of utxos being merged
"mergingTransparentValue" 	(numeric) 	the value of utxos being merged
"mergingNotes" 	(numeric) 	the number of notes being merged
"mergingShieldedValue" 	(numeric) 	the value of notes being merged
"opid" 	(string) 	an operationid to pass to z_getoperationstatus to get the result of the operation

*/

/*
TODO
Implement this feature
Function Name: z_sendmany
Arguments
Name 	Type 	Description
"fromaddress" 	(string, required) 	the sending t address or z address
"amounts" 	(array of json objects)
"address" 	(string, required) 	the receiving address; can be a t address or z address
"amount" 	(numeric, required) 	the numeric amount
"memo" 	(string, optional) 	if the address is a z address, this property accepts raw data represented in hexadecimal string format
minconf 	(numeric, optional, default=1) 	only use funds confirmed at least this many times
fee 	(numeric, optional, default=0.0001) 	the fee amount to attach to this transaction
#
Response
Name 	Type 	Description
"operationid" 	(string) 	an operationid to pass to z_getoperationstatus to get the result of the operation
*/

pub fn z_send_many(
    SomeUser: komodorpcutil::KomodoRPC,
    from_address: String,
    amounts: f32,
    address: String,
    amount: f32,
    memo: Option<String>,
    minconf: Option<u32>,
    fee: Option<f32>,
) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("z_sendmany");
    let method_body: String;
    let temp_fee: f32 = fee.unwrap_or(0.0001);
    //let temp_limit:u32 = limit.unwrap_or(50);

    /* Implement feature here
    method_body = String::from("[\"") +
        &from_address.to_string()+
        &String::from("\", \"")+
        &to_address.to_string()+
        &String::from("\", ")+
        &temp_fee.to_string()+
        &String::from(",")+
        &temp_limit.to_string()+
        &String::from("]");

    */
    //let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
    komodorpcutil::request(SomeUser.clone(), "data".to_string())
}

/*
TODO
Properly test the functionality of the function with properly working
from and to addresses to verify the functionality


Function Name:  z_shieldcoinbase
Arguments
Name 	Type 	Description
"fromaddress" 	(string, required) 	the address is a t address or "*" for all t address belonging to the wallet
"toaddress" 	(string, required) 	the address is a z address
fee 	(numeric, optional, default=0.0001) 	the fee amount to attach to this transaction
limit 	(numeric, optional, default=50) 	limit on the maximum number of utxos to shield; set to 0 to use node option mempooltxinputlimit
#
Response
Name 	Type 	Description
"remainingUTXOs" 	(numeric) 	the number of coinbase utxos still available for shielding
"remainingValue" 	(numeric) 	the value of coinbase utxos still available for shielding
"shieldingUTXOs" 	(numeric) 	the number of coinbase utxos being shielded
"shieldingValue" 	(numeric) 	the value of coinbase utxos being shielded
"opid" 	(string) 	an operationid to pass to z_getoperationstatus to get the result of the operation

*/

pub fn z_shield_coinbase(
    SomeUser: komodorpcutil::KomodoRPC,
    from_address: String,
    to_address: String,
    fee: Option<f32>,
    limit: Option<u32>,
) -> Result<(), reqwest::Error> {
    let method_name: String = String::from("z_shieldcoinbase");
    let method_body: String;
    let temp_fee: f32 = fee.unwrap_or(0.0001);
    let temp_limit: u32 = limit.unwrap_or(50);

    method_body = String::from("[\"")
        + &from_address.to_string()
        + &String::from("\", \"")
        + &to_address.to_string()
        + &String::from("\", ")
        + &temp_fee.to_string()
        + &String::from(",")
        + &temp_limit.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}
