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

*/