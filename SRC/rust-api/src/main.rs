
#![allow(non_snake_case)]
#![allow(dead_code)]
mod komodorpcutil;
mod komodo;
use komodorpcutil::KomodoRPC;




pub fn main() {
    
    let someAddress = String::from("127.0.0.1");
	let somePortNum = 8158;
	let someReqMethod = String::from("POST");
	let someUserName = String::from("user3533350138");
	let somePassword = String::from("pass8e9815842523ea069dd5296a1a3c95f44c65819bd62d6d0d71784336a87a71f580");
	let someJSONRPCVer = String::from("1.0");
	let someRPCReqID = String::from("curltest");
	

	// CHECK TO SEE IF THERE IS USERNAME IS EMPTY???? TODO
	
	// Create KomodoRPC type (an 'object') with variables
	let someUser = KomodoRPC::new(someAddress, somePortNum, someReqMethod,someUserName, somePassword,someJSONRPCVer, someRPCReqID);



//get_block_subsidy
//let get_block_subsidy=komodo::mining::get_block_subsidy(someUser, Some(10)); 

//get_block_template
//let v = vec!["12312".to_string(), "12312".to_string(), "1231232".to_string()];
//let vv = vec!["".to_string()];
//let get_block_template=komodo::mining::get_block_template(someUser, Some("4321312".to_string()), v, "dqwqw".to_string());
//let get_block_template=komodo::mining::get_block_template(someUser, Some("4321312".to_string()), vv, "dqwqw".to_string()); 

//get_local_solps
//let get_local_solps=komodo::mining::get_local_solps(someUser);

//get_mining_info
//let get_mining_info=komodo::mining::get_mining_info(someUser);

//get_network_solps
//let get_network_solps=komodo::mining::get_network_solps(someUser, Some(0), Some(0));
//let get_network_solps=komodo::mining::get_network_solps(someUser, Some(32), Some(0));
//let get_network_solps=komodo::mining::get_network_solps(someUser, Some(32), Some(12));
//let get_network_solps=komodo::mining::get_network_solps(someUser, Some(0), Some(12));

//prioritise_transaction
//let prioritise_transaction=komodo::mining::prioritise_transaction(someUser, "abdefed632747182dbe127312b3e13e1".to_string(), 0, 10000);

//submit_block
//let submit_block=komodo::mining::submit_block(someUser, "adef12341231245678678567adfefdfeabbc".to_string(), Some("jhjg".to_string()));



//decode_raw_transaction
//decode_raw_transaction=komodo::rawtransactions::decode_raw_transaction(someUser, "abdec909180309127461293109def1023984971b142c1230980ac1280124108".to_string());

//decode_script
//decode_script=komodo::rawtransactions::decode_script(someUser, "abdec909180309127461293109def1023984971b142c1230980ac1280124108".to_string());

//fund_raw_transaction
//fund_raw_transaction=komodo::rawtransactions::fund_raw_transaction(someUser, "abdec909180309127461293109def1023984971b142c1230980ac1280124108".to_string());

//get_raw_transaction
//get_raw_transaction=komodo::rawtransactions::get_raw_transaction(someUser, "ade1331ed2556993210".to_string(), 0);
//get_raw_transaction=komodo::rawtransactions::get_raw_transaction(someUser, "ade1331ed2556993210".to_string(), 1);

//send_raw_transaction
//send_raw_transaction=komodo::rawtransactions::send_raw_transaction(someUser, "ade1331ed2556993210".to_string(), true);
//send_raw_transaction=komodo::rawtransactions::send_raw_transaction(someUser, "ade1331ed2556993210".to_string(), false);
}