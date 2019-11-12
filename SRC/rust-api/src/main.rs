mod komodorpcutil;
mod komodo;
use komodorpcutil::KomodoRPC;




pub fn main() {
    
    let someAddress = String::from("127.0.0.1");
	let somePortNum = 8158;
	let someReqMethod = String::from("POST");
	let someUserName = String::from("user1608438106");
	let somePassword = String::from("pass02e12fd396e2434b74e776c19cd03d32d308ff3c104ab23693acd1988610e5f9b4");
	let someJSONRPCVer = String::from("1.0");
	let someRPCReqID = String::from("curltest");
	

	// CHECK TO SEE IF THERE IS USERNAME IS EMPTY???? TODO
	
	// Create KomodoRPC type (an 'object') with variables
	let someUser = KomodoRPC::new(someAddress, somePortNum, someReqMethod,someUserName, somePassword,someJSONRPCVer, someRPCReqID);


//getblockchaininfo
//let blockchain_info= komodo::blockchain::get_blockchain_info(someUser);

//getaddressbalance
let get_address_balance=komodo::address::getaddressbalance(someUser,["RDymhC2RrTKEPmj3rpPUmXeXhJsrTktqbU".to_string()].to_vec()); 




}


