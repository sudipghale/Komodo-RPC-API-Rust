

extern crate reqwest;
use reqwest::header::*;


#[derive(Debug,Clone)]
pub struct KomodoRPC
{
    
    rpc_address: String,
    rpc_port: i32,
	req_method: String,
    rpc_username: String,
    rpc_password: String,
	json_rpc_ver: String,
    rpc_id: String
	
}


impl KomodoRPC
{
	pub fn default() -> KomodoRPC {
        KomodoRPC {
        /*
		rpc_address: String::from("127.0.0.1"),
        rpc_port: 7777,
        req_method: String::from("POST"),
        rpc_username:String::from(""),
        rpc_password:String::from(""),
        json_rpc_ver: String::from("1.0"),
        rpc_id: String::from("curltest"),
		*/
		rpc_address: String::from("127.0.0.1"),
        rpc_port: 8158,
        req_method: String::from("POST"),
        rpc_username:String::from("user2509089925"),
        rpc_password:String::from("passf2e2f61a68e19ce65fe8211aa5d42163fb5c507c8d8d63dabd2c3f66c5358527eb"),
        json_rpc_ver: String::from("1.0"),
        rpc_id: String::from("curltest"),

        }
    }
	
	pub fn new(rpc_address: String, 
	rpc_port: i32, 
	req_method: String,
    rpc_username: String, 
	rpc_password: String,  
	json_rpc_ver: String, 
	rpc_id: String) -> KomodoRPC
	{
		
		KomodoRPC
		{
			
			rpc_address: rpc_address,
			rpc_port: rpc_port,
			req_method: req_method,
			rpc_username: rpc_username,
			rpc_password: rpc_password,
			json_rpc_ver: json_rpc_ver,
			rpc_id: rpc_id
			
		}
	}
		// example helper method for rpc_address
	pub fn GetRPCAddress(&self) -> String
	{
		return self.rpc_address.to_string();
	}

	pub fn GetRPCId(&self) -> String
	{
		return self.rpc_id.to_string();
	}
	
	pub fn GetRequestURL(&self) -> String
	{
		return String::from(format!("http://{0}:{1}/", self.rpc_address, self.rpc_port));
	}	
	
	pub fn GetUsername(&self) -> String
	{
		return self.rpc_username.to_string();
	}
	pub fn GetPassword(&self) -> String
	{
		return self.rpc_password.to_string();
	}

	pub fn GetRequestMetadata(&self) -> String
	{
		let ReqMetadata:String = format!("{{\"jsonrpc\": \"{0}\", \"id\":\"{1}\", ", self.json_rpc_ver, self.rpc_id);
		return ReqMetadata;
	}
	pub fn getJSONRPCver(&self) -> String
	{
		let temp_JSON_RPC_Ver = format!("{}",self.json_rpc_ver);
		return temp_JSON_RPC_Ver;
	}

	

}

pub fn GenerateBody(SomeUser: KomodoRPC, method_name:String , method_parameter:String) -> String
	{
		let method = format!("\"method\":\"{0}\",", method_name);
		let paramater = format! ("\"params\": {0} }}",method_parameter);
	    let mut body:String = SomeUser.GetRequestMetadata();
		body = body + &method + &paramater;
		//body.push_str(method.to_string());
		//body.push_str(paramater);
		//print!("{:?}",body);
		return body;
		
	}



pub fn request(SomeUser: KomodoRPC, body_input: String) -> Result<(), reqwest::Error> {

    
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/plain;".parse().unwrap());

    // assume this is from the struc
	let someJSONRPCVer = String::from(SomeUser.getJSONRPCver());
	let someRPCReqID = String::from(SomeUser.GetRPCId());

    let reqMeta = format!("\"jsonrpc\":\"{0}\", \"id\":\"{1}\"", someJSONRPCVer, someRPCReqID);
    //                  "jsonrpc": "1.0", "id":"curltest"
    //println!("\n\nthe value of asdf is: {}\n", reqMeta);
	let  URL_POST= SomeUser.GetRequestURL();


    let res = reqwest::Client::new()
        .post(&URL_POST)
        .basic_auth(SomeUser.GetUsername(), Some(SomeUser.GetPassword()))
        .headers(headers)
        //.body("{\"jsonrpc\": \"1.0\", \"id\":\"curltest\", \"method\": \"getwalletinfo\", \"params\": [] }")
        .body(body_input)        
        .send()?
        .text()?;
    println!("{}", res);
    Ok(())
}



