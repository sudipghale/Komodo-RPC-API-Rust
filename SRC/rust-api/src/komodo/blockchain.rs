use super:: komodorpcutil;
use komodorpcutil::KomodoRPC;



//getblockchaininfo

pub fn get_blockchain_info(SomeUser: komodorpcutil::KomodoRPC) ->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getblockchaininfo");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::GenerateBody(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;

}
	