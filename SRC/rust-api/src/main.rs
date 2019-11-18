#![allow(non_snake_case)]
#![allow(dead_code)]


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
//let get_address_balance=komodo::address::get_address_balance(someUser,["RFK5paVBsRdpdzc9wDYuNxAhmz61668Npr".to_string()].to_vec());

//getaddressdeltas
//let get_address_deltas=komodo::address::get_address_deltas(someUser,["RDymhC2RrTKEPmj3rpPUmXeXhJsrTktqbU".to_string()].to_vec(),1,200,true);

//getaddressmempool
//let get_address_deltas=komodo::address::get_address_mem_pool(someUser,["RDymhC2RrTKEPmj3rpPUmXeXhJsrTktqbU".to_string()].to_vec());

//getaddresstxids
//let get_address_txids=komodo::address::get_address_txids(someUser,["RDymhC2RrTKEPmj3rpPUmXeXhJsrTktqbU".to_string()].to_vec(),1,200);

//getaddressutxos
//let get_address_utxos=komodo::address::get_address_utxos(someUser,["RDymhC2RrTKEPmj3rpPUmXeXhJsrTktqbU".to_string()].to_vec(),true);

//getsnapshot
//let get_snapshoot=komodo::address::get_snapshot(someUser,Some(5));

//getinfo
//let get_info=komodo::control::get_info(someUser);

//help
//let help=komodo::control::help(someUser,Some("getaddressbalance".to_string()));// or None

//stop
//let stop=komodo::control::stop(someUser);

//z_getpaymentdisclosure
//let reusult=komodo::disclosure::z_get_payment_disclosure(someUser,"96f12882450429324d5f3b48630e3168220e49ab7b0f066e5c2935a6b88bb0f2".to_string(),
 //"0".to_string(), "0".to_string(), Some("refund".to_string()));

//getaddressbalance
//let get_address_balance=komodo::address::getaddressbalance(someUser,["RJ2j4HuHMERjY9kR81Kdo1KhCKV2dpPArs".to_string()].to_vec());

 //z_validatepaymentdisclosure
 //let reuslt= komodo::disclosure::z_validate_payment_disclosure(someUser,"zpd:706462ff004c561a0447ba2ec51184e6c20".to_string());

 //generate
 //let result = komodo::generate::generate(someUser,2);

  //get_generate
  //let result = komodo::generate::get_generate(someUser);

   //get_generate 
   //let result = komodo::generate::set_generate(someUser,true,Some(4)); //test1
   //let result = komodo::generate::set_generate(someUser,true,None); //test2
	// let result = komodo::generate::set_generate(someUser,true,None); //test3
	 //let result = komodo::generate::set_generate(someUser,false,None); //test4

//addnode
//let result= komodo::network::add_node(someUser,"1.1.1.1:8899".to_string(),"onetry".to_string());

//clearbanned
//let result= komodo::network::clear_banned(someUser);

//disconnectnode
//let result= komodo::network::disconnect_node(someUser,"1.1.1.1:8899".to_string());


//getaddednodeinfo
//let result= komodo::network::get_added_node_info(someUser,true,None);

//getconnectioncount
//let result= komodo::network::get_connection_count(someUser);

//getdeprecationinfo
//let result= komodo::network::get_deprecation_info(someUser);

//getnettotals
//let result= komodo::network::get_net_totals(someUser);

//getnetworkinfo
//let result= komodo::network::get_network_info(someUser);

//getpeerinfo
//let result= komodo::network::get_peer_info(someUser);

//listbanned
//let result= komodo::network::list_banned(someUser);

//ping
//let result= komodo::network::ping(someUser);

//setban
//let result= komodo::network::set_ban(someUser,"1.1.1.1".to_string(),"add".to_string(), Some(86400), None);

//backupwallet
//let result= komodo::wallet::backup_wallet(someUser,"mybackupdata".to_string());

//dumpprivkey
//let result= komodo::wallet::dump_priv_key(someUser,"smfslfnsfsdfdskj".to_string());

//dumpwallet
let result = komodo::wallet::dump_wallet(someUser,"filename".to_string());


}

