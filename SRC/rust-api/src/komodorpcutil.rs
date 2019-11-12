extern crate reqwest;
use reqwest::header::*;

#[derive(Debug,Clone)]
/**
 * Struct Komodo RPC holds the configuration information 
 * to make connection to the komodo Daemon
 */
pub struct KomodoRPC
{
	rpc_address: String, 		// the ip address/URL of the komodo daemon
    rpc_port: i32, 				// RPC port number of Komodo Daemon Server
	req_method: String,			// Request method such as GET, POST	
    rpc_username: String,		// Username of the account in Komodo Daemon for authentication
    rpc_password: String,		// Password of the account in Komodo Daemon for authentication
	json_rpc_ver: String,		// JSON version
    rpc_id: String				// RPC request ID for the connection
}


/**
 * Implementing methods for Komodo RPC Configuration Struct. 
 * The methods will read and write data to the Komodo RPC Configuration Struct instance.
 *  */
impl KomodoRPC
{
	/**
	 * The default method of Komodo RPC initializes Komodo RPC with 
	 * default address, port, method, username, passoword, RPC version, RPC ID
	 */
	pub fn default() -> KomodoRPC {
        KomodoRPC {
        	rpc_address: String::from("127.0.0.1"),
			rpc_port: 8158,
	        req_method: String::from("POST"),
	        rpc_username:String::from("user2509089925"),
	        rpc_password:String::from("passf2e2f61a68e19ce65fe8211aa5d42163fb5c507c8d8d63dabd2c3f66c5358527eb"),
	        json_rpc_ver: String::from("1.0"),
	        rpc_id: String::from("curltest"),
        }
    }

	/**
	 * The new method of KomodoRPC takes input paramaters from the calee
	 * and assigns the input to the struct Komodo RPC. 
	 */
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
			//Assigning the paramaters to the struct 
			rpc_address: rpc_address,
			rpc_port: rpc_port,
			req_method: req_method,
			rpc_username: rpc_username,
			rpc_password: rpc_password,
			json_rpc_ver: json_rpc_ver,
			rpc_id: rpc_id,
		}
	}

	/**
	 *Function Name: get_rpc_address
	 *@params: the instance of the struct KomodoRPC
	 *Output: string that has RPC address such as URL or IP address
	 */
	pub fn get_rpc_address(&self) -> String
	{
		return self.rpc_address.to_string();
	}

	/**
	 *Function Name: get_rpc_id
	 *@params: the instance of the struct KomodoRPC
	 *Output: string that has RPC ID
	 */
	pub fn get_rpc_id(&self) -> String
	{
		return self.rpc_id.to_string();
	}

	/**
	 *Function Name: get_request_url
	 *@params: the instance of the Struct KomodoRPC
	 *Output: string that has Request URL and port number in format http://72.168.1.44:22
	 */
	pub fn get_request_url(&self) -> String
	{
		return String::from(format!("http://{0}:{1}/", self.rpc_address, self.rpc_port));
	}	
	
	/**
	 *Function Name: get_username
	 *@params: the instance of the struct KomodoRPC
	 *Output: string that has rpc_address
	 */
	pub fn get_username(&self) -> String
	{
		return self.rpc_username.to_string();
	}

	/**
	 *Function Name: get_password
	 *@params: the instance of the struct KomodoRPC
	 *Output: string that has the password
	 */
	pub fn get_password(&self) -> String
	{
		return self.rpc_password.to_string();
	}

	/**
	 *Function Name: get_request_metadata
	 *@params: the instance of the struct KomodoRPC
	 *Output: string that has RPC request meta data in following format
	 *{"jsonrpc": "1", "id" : "curltest"}
	 */
	pub fn get_request_metadata(&self) -> String
	{
		let request_metadata:String = format!("{{\"jsonrpc\": \"{0}\", \"id\":\"{1}\", ", self.json_rpc_ver, self.rpc_id);
		return request_metadata;
	}
	/**
	 *Function Name: get_json_ver
	 *@params: the instance of the struct KomodoRPC
	 *Output: string containing the json version for the request reply
	 */
	pub fn get_json_ver(&self) -> String
	{
		let temp_json_rpc_ver = format!("{}",self.json_rpc_ver);
		return  temp_json_rpc_ver;
	}

}

/**
	 *Function Name: generate_body
	 *@params: KomodoRPC - the instance of the struct KomodoRPC
	 			method_name - name of the method to query to the Komodo Daemon
				 method_paramaters - paramaters for the method passed 
	 *Output: the function generates an output for the HTTP request method encoding
	 * 			the method, paramaters and metadata generated by get_request_metadata() function.
	 */
pub fn generate_body(some_user: KomodoRPC, method_name:String , method_parameter:String) -> String
	{
		let method = format!("\"method\": \"{0}\", ", method_name);
		let paramater = format! ("\"params\": {0} }}",method_parameter);
	    let mut body:String = some_user.get_request_metadata();
		body = body + &method + &paramater;
		//body.push_str(method.to_string());
		//body.push_str(paramater);
		//print!("{:?}",body);
		return body;
	}

	/**
	 *Function Name: request
	 *@params: some_user - an instance of the KomodoRPC struct containing the configuration for Komodo RPC Request
	 			body_input - it has the body of the request provided by the get_body() method.
	 *Output: Result of the request 
	 			ie. JSON containing the the output result from the request 
				 	Error Codes - containing the error in the HTTP request
	 */

pub fn request(some_user: KomodoRPC, body_input: String) -> Result<(), reqwest::Error> {
	if some_user.get_username().trim() !=""
	{	
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/plain;".parse().unwrap());
    // assume this is from the struc
	let some_json_rpc_ver = String::from(some_user.get_json_ver());
	let some_rpc_id = String::from(some_user.get_rpc_id());
	let  url_post= some_user.get_request_url();
    let res = reqwest::Client::new()
        .post(&url_post)
        .basic_auth(some_user.get_username(), Some(some_user.get_password()))
        .headers(headers)
        //.body("{\"jsonrpc\": \"1.0\", \"id\":\"curltest\", \"method\": \"getwalletinfo\", \"params\": [] }")
        .body(body_input)        
        .send()?
        .text()?;
    println!("the result is{}", res);
    Ok(())
	}
	else
	{
		println!("Error");
		return Ok(());
	}
}