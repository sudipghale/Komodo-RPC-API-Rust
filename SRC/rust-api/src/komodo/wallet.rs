use super:: komodorpcutil;
use komodorpcutil::KomodoRPC;

//addmultisigaddress has been DEPRECATED

/*
backupwallet
backupwallet "destination"

The backupwallet method safely copies the wallet.dat file to the indicated destination. The destination input accepts only alphanumeric characters.

This method requires that the coin daemon have the exportdir runtime parameter enabled.

#Arguments
Name	Type	Description
"destination"	(string, required)	the destination filename, saved in the directory set by the exportdir runtime parameter
#Response
Name	Type	Description
"path"	(string)	the full path of the destination file
*/
pub fn backup_wallet(someUser:komodorpcutil::KomodoRPC, destination:String)->Result<(),reqwest::Error>
{
  
let method_body =String::from("[\"") + &destination+ "\"]";// String::from(format!("[/"{0}/"]",temp_top));
let method_name:String = String::from("backupwallet"); 
let data:String = String::from (komodorpcutil::generate_body(someUser.clone(),method_name,method_body));

println!("the body is{:?}",data );
let result = komodorpcutil::request( someUser.clone(), data);
return result;
}

/*
dumpprivkey
dumpprivkey "address"

The dumpprivkey method reveals the private key corresponding to the indicated address.

See also importprivkey.

#Arguments
Name	Type	Description
"address"	(string, required)	the address for the private key
#Response
Name	Type	Description
"data"	(string)	the private key

*/
pub fn dump_priv_key(someUser:komodorpcutil::KomodoRPC, address:String)->Result<(),reqwest::Error>
{
  
let method_body =String::from("[\"") + &address+ "\"]";// String::from(format!("[/"{0}/"]",temp_top));
let method_name:String = String::from("dumpprivkey"); 
let data:String = String::from (komodorpcutil::generate_body(someUser.clone(),method_name,method_body));

println!("the body is{:?}",data );
let result = komodorpcutil::request( someUser.clone(), data);
return result;
}

/*
dumpwallet
dumpwallet "filename"

The dumpwallet method dumps all transparent-address wallet keys into a file, using a human-readable format.

Overwriting an existing file is not permitted. The destination parameter accepts only alphanumeric characters.

This method requires that the coin daemon have the exportdir runtime parameter enabled.

#Arguments
Name	Type	Description
"filename"	(string, required)	the filename, saved in the folder set by the exportdir runtime parameter
Response
Name	Type	Description
"path"	(string)	the full path of the destination file

*/

pub fn dump_wallet(someUser:komodorpcutil::KomodoRPC, filename:String)->Result<(),reqwest::Error>
{
  
let method_body =String::from("[\"") + &filename+ "\"]";// String::from(format!("[/"{0}/"]",temp_top));
let method_name:String = String::from("dumpwallet"); 
let data:String = String::from (komodorpcutil::generate_body(someUser.clone(),method_name,method_body));

println!("the body is{:?}",data );
let result = komodorpcutil::request( someUser.clone(), data);
return result;
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
Name	Type	Description
passphrase	(string)	the passphrase for wallet encryption; the passphrase must be at least 1 character, but should be many
#Response
Text Response
wallet encrypted; Komodo server stopping, restart to run with encrypted wallet. The keypool has been flushed, you need to make a new backup

*/
pub fn encrypt_wallet(someUser:komodorpcutil::KomodoRPC, passphrase:String)->Result<(),reqwest::Error>
{
  
let method_body =String::from("[\"") + &passphrase+ "\"]";// String::from(format!("[/"{0}/"]",temp_top));
let method_name:String = String::from("encruptwallet"); 
let data:String = String::from (komodorpcutil::generate_body(someUser.clone(),method_name,method_body));

println!("the body is{:?}",data );
let result = komodorpcutil::request( someUser.clone(), data);
return result;
}
/*
 * getaccount
getaccount "address"

The getaccount method returns the account associated with the given address.

#Arguments
Name	Type	Description
"address"	(string, required)	the address
#Response
Name	Type	Description
"accountname"	(string)	the account address
 * 
 */
 
 pub fn get_account(someUser:komodorpcutil::KomodoRPC, address:String)->Result<(),reqwest::Error>
{
  
let method_body =String::from("[\"") + &address+ "\"]";// String::from(format!("[/"{0}/"]",temp_top));
let method_name:String = String::from("getaccount"); 
let data:String = String::from (komodorpcutil::generate_body(someUser.clone(),method_name,method_body));

println!("the body is{:?}",data );
let result = komodorpcutil::request( someUser.clone(), data);
return result;
}

/*
getbalance
getbalance ( "account" minconf includeWatchonly )

The getbalance method returns the server's total available balance.

The account input is deprecated.

#Arguments
Name	Type	Description
"account"	(string, optional)	DEPRECATED if provided, it MUST be set to the empty string "" or to the string "*"
minconf	(numeric, optional, default=1)	only include transactions confirmed at least this many times
includeWatchonly	(bool, optional, default=false)	also include balance in watchonly addresses (see importaddress)

#Response
Name	Type	Description
amount	(numeric)	the total amount
*/
pub fn get_balance(  //TODO check def value and if conditions 
    SomeUser: komodorpcutil::KomodoRPC,
    minconf:Option<u32>,
    includeWatchonly:Option<bool>)
      ->Result<(), reqwest::Error>
  {
      let method_body:String;
      let temp_minconf = minconf.unwrap_or(1); //Default value is 1
      let temp_includeWatchonly:String = includeWatchonly.unwrap_or(false).to_string();
      if (!(temp_minconf==1) && !(temp_includeWatchonly.is_empty()))
      {
        method_body = String::from("[")+ &temp_minconf.to_string()+ &",".to_string()+ &temp_includeWatchonly+ &String::from("]");
      }
      else if ((temp_minconf==1) && !(temp_includeWatchonly.is_empty()))
      {
        method_body = String::from("[") + &temp_minconf.to_string()+  &String::from("]");
     }
     else if (!(temp_minconf==1) && (temp_includeWatchonly.is_empty()))
      {
        method_body = String::from("[") + &temp_includeWatchonly.to_string()+  &String::from("]");
     }
     else//  ((temp_bantime==0) && !(temp_absolute.is_empty()))
      {
      method_body = String::from("[") + &String::from("]");	
      }
  
      let method_name:String = String::from("getbalance");
        let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
        let result =komodorpcutil::request( SomeUser.clone(), data);
      return result;
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
pub fn get_new_address(SomeUser: komodorpcutil::KomodoRPC) ->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getnewaddress");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
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
pub fn get_raw_change_address(SomeUser: komodorpcutil::KomodoRPC) ->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getrawchangeaddress");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
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
	address:String,
	min_conf:Option<u32>) 
	->Result<(), reqwest::Error>
{
	let method_body:String;
	let temp_min_conf:String = min_conf.unwrap_or(1).to_string();// default 1///TO DO
    if (temp_min_conf.is_empty())
    {
		method_body = String::from("[\"") + &address.to_string()+ &String::from("\"")+ &String::from("]");
    }
    else//if (!temp_gen_proc_limit.is_empty())
    {
		method_body = String::from("[\"") + &address.to_string()+ &"\",".to_string()+ &temp_min_conf +&String::from("]");
	}
	
    let payload =  "{\"jsonrpc\": \"1.0\", \"id\":\"curltest\",  \"method\": \"getreceivedbyaddress\", \"params\": [\"RJSDZjp7kjBNhHsbECDE1jwYNK7af41pZN\", 6] }\n";
    let method_name:String = String::from("getreceivedbyaddress");
    let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
    println!("payload is {:?}", payload);
    println!("the body is{:?}",data );
	let result =komodorpcutil::request( SomeUser.clone(), data);
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
	include_watch_only:Option<bool>) 
	->Result<(), reqwest::Error>
{
	let method_body:String;
  let temp_include_watch_only:String = include_watch_only.unwrap_or(false).to_string();
  method_body = String::from("[\"") + &tx_id.to_string()+ &String::from("\",")+&temp_include_watch_only+ &String::from("]");
  let method_name:String = String::from("gettransaction");
  let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
  println!("the body is{:?}",data );
	let result =komodorpcutil::request( SomeUser.clone(), data);
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
pub fn get_unconfirmed_balance(SomeUser: komodorpcutil::KomodoRPC) ->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getunconfirmedbalance");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
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
pub fn get_wallet_info(SomeUser: komodorpcutil::KomodoRPC) ->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getwalletinfo");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
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
pub fn import_address(  //TODO check def value and if conditions 
  SomeUser: komodorpcutil::KomodoRPC,
  address:String,
  label:Option<String>,
  rescan:Option<bool>)
    ->Result<(), reqwest::Error>
{
    let method_body:String;
    let temp_label:String = label.unwrap_or("".to_string()).to_string(); 
    let temp_rescan:String = rescan.unwrap_or(true).to_string();

    if(temp_label.is_empty())
    {
      method_body = String::from("[\"")+ &address+ &"\",".to_string()+ &temp_rescan+ &String::from("]");
    }
    else{
      method_body = String::from("[\"")+ &address+ &"\",\"".to_string()+ &temp_label+&"\",".to_string()+ &temp_rescan+ &String::from("]");
        }
    let method_name:String = String::from("importaddress");
    let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
    let result =komodorpcutil::request( SomeUser.clone(), data);
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
pub fn import_priv_key(  //TODO check def value and if conditions 
  SomeUser: komodorpcutil::KomodoRPC,
  priv_key:String,
  label:Option<String>,
  rescan:Option<bool>)
    ->Result<(), reqwest::Error>
{
    let method_body:String;
    let temp_label:String = label.unwrap_or("".to_string()).to_string(); 
    let temp_rescan:String = rescan.unwrap_or(true).to_string();

    if(temp_label.is_empty())
    {
      method_body = String::from("[\"")+ &priv_key+ &"\",".to_string()+ &temp_rescan+ &String::from("]");
    }
    else{
      method_body = String::from("[\"")+ &priv_key+ &"\",\"".to_string()+ &temp_label+&"\",".to_string()+ &temp_rescan+ &String::from("]");
        }
    let method_name:String = String::from("importprivkey");
    let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
    println!("the body is{:?}",data );

    let result =komodorpcutil::request( SomeUser.clone(), data);
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

pub fn import_wallet(someUser:komodorpcutil::KomodoRPC,file_name:String)-> Result<(),reqwest::Error>{

  let params =String::from("[\"") + &file_name+ "\"]";

      
  let method_name:String = String::from("importwallet");
      let method_body:String = String::from(params);
      let data:String = String::from (komodorpcutil::generate_body(someUser.clone(),method_name,method_body));
      println!("the body is{:?}",data );
      let result = komodorpcutil::request( someUser.clone(), data);
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
	new_size:Option<u32>) 
	->Result<(), reqwest::Error>
{
	let method_body:String;
  let temp_new_size:String = new_size.unwrap_or(100).to_string();
  method_body = String::from("[") +&temp_new_size+ &String::from("]");
  let method_name:String = String::from("keypoolrefill");
  let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
  println!("the body is{:?}",data );
	let result =komodorpcutil::request( SomeUser.clone(), data);
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
pub fn list_address_groupings(SomeUser: komodorpcutil::KomodoRPC) ->Result<(), reqwest::Error>
{
    let method_name:String = String::from("listaddressgroupings");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;

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
pub fn list_lock_unspent(SomeUser: komodorpcutil::KomodoRPC) ->Result<(), reqwest::Error>
{
    let method_name:String = String::from("listlockunspent");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;

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
  min_conf:Option<u32>,
  include_empty:Option<bool>, 
  include_watch_only:Option<bool>
) 
	->Result<(), reqwest::Error>
{
  let method_body:String;
  let temp_min_conf:String = min_conf.unwrap_or(1).to_string();
  let temp_include_empty:String = include_empty.unwrap_or(false).to_string();
  let temp_include_watch_only:String = include_watch_only.unwrap_or(false).to_string();
  
  method_body = String::from("[") + &temp_min_conf+&String::from(",")+ &temp_include_empty+ &String::from(",")+&temp_include_watch_only+ &String::from("]");
  let method_name:String = String::from("listreceivedbyaddress");
  let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
  println!("the body is{:?}",data );
	let result =komodorpcutil::request( SomeUser.clone(), data);
  return result;
}

/**
 * listsinceblock
listsinceblock ( "blockhash" target-confirmations includeWatchonly )

The listsinceblock method queries all transactions in blocks since block blockhash, or all transactions if blockhash is omitted.

#Arguments
Name	Type	Description
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