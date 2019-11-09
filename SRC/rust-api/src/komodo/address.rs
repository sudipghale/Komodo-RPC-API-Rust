use super:: komodorpcutil;
use komodorpcutil::KomodoRPC;



//getblockchaininfo

pub fn getaddressbalance(someUser: komodorpcutil::KomodoRPC,v_address:Vec<String>) -> Result<(), reqwest::Error> {      // implement for vec of addrs

let mut addr_list = String::from("[");

for addr in &v_address{ 
    addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
}

if(v_address.len() >0) // >1
{
    // why need to cut last substring
   addr_list = addr_list[0..(addr_list.len()-1)].to_string();
}

addr_list = addr_list + &"]"; //& for String -> &string

let method_name:String = String::from("getaddressbalance");
	let method_body:String = String::from(addr_list);
	let data:String = String::from (komodorpcutil::generate_body(someUser.clone(),method_name,method_body));
	println!("the body is{}",data );
	let result = komodorpcutil::request( someUser.clone(), data);
    return result;
    

}