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
Name	Type	Description
"address"	(string, required)	the address for transactions
minconf	(numeric, optional, default=1)	only include transactions confirmed at least this many times
#Response
Name	Type	Description
amount	(numeric)	the total amount of the relevant coin received at this address
*/