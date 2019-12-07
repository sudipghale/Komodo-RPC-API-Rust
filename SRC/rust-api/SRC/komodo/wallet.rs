use super::komodorpcutil;
use komodorpcutil::KomodoRPC;



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
