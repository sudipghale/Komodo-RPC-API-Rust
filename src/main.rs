#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(warnings)]
mod komodo;
mod komodorpcutil;
use komodorpcutil::KomodoRPC;

pub fn main() {
    let someAddress = String::from("127.0.0.1");
    let somePortNum = 13211;
    let someReqMethod = String::from("POST");
    let someUserName = String::from("user629608817");
    let somePassword =
        String::from("pass6375b5f55e1b166f744d7c66b66354217a6d36645d1de5302c8780bca9600cce49");
    let someJSONRPCVer = String::from("1.0");
    let someRPCReqID = String::from("curltest");

    // CHECK TO SEE IF THERE IS USERNAME IS EMPTY???? TODO

    // Create KomodoRPC type (an 'object') with variables
    let someUser = KomodoRPC::new(
        someAddress,
        somePortNum,
        someReqMethod,
        someUserName,
        somePassword,
        someJSONRPCVer,
        someRPCReqID,
    );
    //let someUser = KomodoRPC::default();

    //getblockchaininfo
    //let blockchain_info= komodo::blockchain::get_blockchain_info(someUser);

    //getaddressbalance

    let something = komodo::wallet::send_to_address(
        someUser,
        "RKL5sZabVmngF5ueG8hBD1zLFauTqnHFuk".to_string(),
        10.0,
        Some("kenny".to_string()),
        Some("alfonso".to_string()),
        Some(true),
    );
    /*let something = komodo::wallet::get_balance(
        someUser,
        None,
        None,
    );
    */
    println!("{:?}", something);
}
