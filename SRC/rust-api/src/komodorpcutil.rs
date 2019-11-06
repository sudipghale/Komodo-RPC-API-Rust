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
	req_auth: Auth,
	req_url: String,
	req_headers: String,
	json_rpc_ver: String,
    rpc_id: String
	
}

// the structure for user Authentication, variables are private by default
#[derive(Debug,Clone)]
pub struct Auth
{
    rpc_username: String,
    rpc_password: String
}

// constructor for KomodoRPC that initilizes given variables
impl KomodoRPC
{
	
	pub fn new(rpc_address: String, rpc_port: i32, req_method: String,
    rpc_username: String, rpc_password: String, req_auth: Auth, req_url: String,
	req_headers: String, json_rpc_ver: String, rpc_id: String) -> KomodoRPC
	{
		
		KomodoRPC
		{
			
			rpc_address: rpc_address,
			rpc_port: rpc_port,
			req_method: req_method,
			rpc_username: rpc_username,
			rpc_password: rpc_password,
			req_auth: req_auth,
			req_url: req_url,
			req_headers: req_headers,
			json_rpc_ver: json_rpc_ver,
			rpc_id: rpc_id
			
		}
	}
		// example helper method for rpc_address
	pub fn GetRPCAddress(&self) -> String
	{
		return self.rpc_address.to_string();
	}
	
	pub fn GetHeader(&self) -> String
	{
		return self.req_headers.to_string();
	}
	pub fn GetRequestURL(&self) -> String
	{
		return self.req_url.to_string();
	}	
	
	pub fn GetUsername(&self) -> String
	{
		return self.req_auth.rpc_username.to_string();
	}
	pub fn GetPassword(&self) -> String
	{
		return self.req_auth.rpc_password.to_string();
	}

	pub fn GetRequestMetadata(&self) -> String
	{
		let ReqMetadata:String = format!("{{\"jsonrpc\": \"{0}\", \"id\":\"{1}\", ", self.json_rpc_ver, self.rpc_id);
		return ReqMetadata;
	}
	pub fn getJSONRPCver(&self) -> String
	{
		let temp_JSON_RPC_Ver: String = format!("{}",self.json_rpc_ver); // added String
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
		println!("THE BODY : {:?}",body);
		return body;
		
	}


// constructor for Auth that initilizes given variables

impl Auth
{
	
	pub fn new(username: String, password: String) -> Auth
	{
		
		Auth
		{
			
			rpc_username: username,
			rpc_password: password
			
		}
		
	}
	
}



pub fn request(SomeUser: KomodoRPC, body_input: String) -> Result<(), reqwest::Error> {
    
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, SomeUser.GetHeader().parse().unwrap());

    // assume this is from the struc
	let someJSONRPCVer = String::from(SomeUser.getJSONRPCver()); // no need String
	let someRPCReqID = String::from("curltest");

	let  URL_POST= SomeUser.GetRequestURL();


    let res = reqwest::Client::new()
        .post(&URL_POST)
        .basic_auth(SomeUser.GetUsername(), Some(SomeUser.GetPassword()))
        .headers(headers)
        .body(body_input)        
        .send()?
        .text()?;
    println!("THE RESULT IS={:#?}", res);
    Ok(())
}
