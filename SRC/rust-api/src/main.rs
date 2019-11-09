
mod komodorpcutil;
use komodorpcutil::KomodoRPC;


pub fn main() {
    
	
	let someAddress = String::from("127.0.0.1");
	let somePortNum = 8158;
	let someReqMethod = String::from("POST");
	let someUserName = String::from("user2509089925");
	let somePassword = String::from("passf2e2f61a68e19ce65fe8211aa5d42163fb5c507c8d8d63dabd2c3f66c5358527eb");
	
	let someJSONRPCVer = String::from("1.0");
	let someRPCReqID = String::from("curltest");
	
	// Create KomodoRPC type (an 'object') with variables
	//let someUser = KomodoRPC::new(someAddress, somePortNum, someReqMethod, someUserName, somePassword, someJSONRPCVer, someRPCReqID);
	// fn getblockchaininfo(someUser: KomodoRPC, param1:string , prarm : string)
	//{

		let someUser = KomodoRPC::default();
	let method_name:String = String::from("getblockchaininfo");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(someUser.clone(),method_name,method_body));
	
	let result = komodorpcutil::request( someUser.clone(), data);
	//}
	print!("{:?}",result);

    
}




