
#![allow(non_snake_case)]
#![allow(dead_code)]
mod komodorpcutil;
mod komodo;
use komodorpcutil::KomodoRPC;




pub fn main() {
    
    let someAddress = String::from("127.0.0.1");
	let somePortNum = 8158;
	let someReqMethod = String::from("POST");
	let someUserName = String::from("user2509089925");
	let somePassword = String::from("passf2e2f61a68e19ce65fe8211aa5d42163fb5c507c8d8d63dabd2c3f66c5358527eb");
	let someJSONRPCVer = String::from("1.0");
	let someRPCReqID = String::from("curltest");
	

	// CHECK TO SEE IF THERE IS USERNAME IS EMPTY???? TODO
	
	// Create KomodoRPC type (an 'object') with variables
	let someUser = KomodoRPC::new(someAddress, somePortNum, someReqMethod,someUserName, somePassword,someJSONRPCVer, someRPCReqID);


//getblockchaininfo
//let blockchain_info= komodo::blockchain::get_blockchain_info(someUser);

//getaddressbalance
let get_address_balance=komodo::address::getaddressbalance(someUser,["RJ2j4HuHMERjY9kR81Kdo1KhCKV2dpPArs".to_string()].to_vec()); 




}