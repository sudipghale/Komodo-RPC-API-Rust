use super:: komodorpcutil;
use komodorpcutil::KomodoRPC;



/*getaddressbalance 
getaddressbalance '{ "addresses" : [ "address" , ... ] }'

The getaddressbalance method returns the confirmed balance for an address, or addresses. It requires addressindex to be enabled.

#Arguments
Name	     Type      	Description
"address"	(string)	the address

# Response
Name	    Type	     Description
"balance"	(number)	the current confirmed balance in satoshis
"received"	(number)	the total confirmed number of satoshis received (including change)
*/
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

//BODY HAS TO BE
//"{\"jsonrpc\": \"1.0\", \"id\":\"curltest\", \"method\": \"getaddressbalance\", \"params\": [{\"addresses\": [\"RDymhC2RrTKEPmj3rpPUmXeXhJsrTktqbU\"]}] }
let method_name:String = String::from("getaddressbalance");
	let method_body:String = String::from(addr_list);
	let data:String = String::from (komodorpcutil::generate_body(someUser.clone(),method_name,method_body));
	println!("the body is{}",data );
	let result = komodorpcutil::request( someUser.clone(), data);
    return result;
    

}


/*

getaddressdeltas
getaddressdeltas '{ "addresses" : [ "address" , ... ] }'

getaddressdeltas '{ "addresses" : [ "address" , ... ] , "start": start, "end": end, "chainInfo": boolean }'

The getaddressdeltas method returns all confirmed balance changes of an address. The user can optionally limit the response to a given interval of blocks. The method requires addressindex to be enabled.

Arguments
Name	    Type	    Description
"address"	(string)	the address
"start"  	(number)	the start block height
"end"	    (number)	the end block height
"chainInfo"	(boolean)	include chain info in results (only applies if start and end specified)

# Response
Name	    Type	    Description
"satoshis"	(number)	the difference in satoshis
"txid"	(string)	the related transaction id
"index"	(number)	the related input or output index
"height"	(number)	the block height
"address"	(string)	the address

*/ 
pub fn getaddressdeltas(someUser: komodorpcutil::KomodoRPC,v_address:Vec<String>, start:u32, end:u32,chainInfo:bool) -> Result<(), reqwest::Error> { 

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
    
    let method_name:String = String::from("getaddressdeltas");
        let method_body:String = String::from(addr_list);
        let data:String = String::from (komodorpcutil::generate_body(someUser.clone(),method_name,method_body));
        println!("the body is{}",data );
        let result = komodorpcutil::request( someUser.clone(), data);
        return result;
        
    
    }
 