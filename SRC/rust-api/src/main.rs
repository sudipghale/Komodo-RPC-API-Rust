mod komodorpcutil;
mod komodo;
use komodorpcutil::KomodoRPC;




pub fn main() {
    
    let someAddress = String::from("127.0.0.1");
	let somePortNum = 15129;
	let someReqMethod = String::from("POST");
	let someUserName = String::from("user1147343206");
	let somePassword = String::from("passbdbf70f00570bde257e25cf4c3a6b16d903b5aed84486c337d66379449f4963b62");
	let someJSONRPCVer = String::from("1.0");
	let someRPCReqID = String::from("curltest");
	

	// CHECK TO SEE IF THERE IS USERNAME IS EMPTY???? TODO
	
	// Create KomodoRPC type (an 'object') with variables
	let someUser = KomodoRPC::new(someAddress, somePortNum, someReqMethod,someUserName, somePassword,someJSONRPCVer, someRPCReqID);


//getblockchaininfo
//let blockchain_info= komodo::blockchain::get_blockchain_info(someUser);

//getaddressbalance
let get_address_balance=komodo::address::getaddressbalance(someUser,["RDymhC2RrTKEPmj3rpPUmXeXhJsrTktqbU".to_string(),"AAAAAA".to_string()].to_vec()); 




}


