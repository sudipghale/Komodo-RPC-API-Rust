mod komodorpcutil;
mod address;
use komodorpcutil::Auth;
use komodorpcutil::KomodoRPC;




pub fn main() {
    
	
	let someAddress = String::from("127.0.0.1");
	let somePortNum = 12545;
	let someReqMethod = String::from("POST");
	let someUserName = String::from("user3820554962");
	let somePassword = String::from("passaf51366d26a7c781b89b80d4fd24cfb9a2c2580750496265fa1dd3478bd273f2aa");
	let someAuth = Auth::new(someUserName.to_string(), somePassword.to_string());
	let someReqURL = String::from(format!("http://{0}:{1}/", someAddress, somePortNum));
   	let someReqHeader = String::from("");
	let someJSONRPCVer = String::from("1.0");
	let someRPCReqID = String::from("curltest");

	// CHECK TO SEE IF THERE IS USERNAME IS EMPTY???? TODO
	
	// Create KomodoRPC type (an 'object') with variables
	let someUser = KomodoRPC::new(someAddress, somePortNum, someReqMethod, 
	someUserName, somePassword, someAuth, someReqURL, someReqHeader, 
	someJSONRPCVer, someRPCReqID);


//getwalletinfo
		
	let method_name:String = String::from("getwalletinfo");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::GenerateBody(someUser.clone(),method_name,method_body));
	
	let result = komodorpcutil::request( someUser.clone(), data);



}


