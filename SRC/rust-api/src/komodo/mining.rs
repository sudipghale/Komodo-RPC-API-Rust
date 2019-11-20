use super:: komodorpcutil;
use komodorpcutil::KomodoRPC;

/*The get_block_subsidy method returns the block-subsidy reward. 
The resulting calculation takes into account the mining slow start. 
This method can be used in conjunction with custom mining rewards designed by the developers of a KMD-based Smart Chain.

# Arguments
Name	    Type	            Description
"height"	(u32, optional)	    the block height 

# Response
Name	    Type     	        Description
"miner" 	(u32)	            the mining reward amount
*/ 

pub fn get_block_subsidy(
    some_user: komodorpcutil::KomodoRPC,
    height_supplied: Option<u32>)
     -> Result<(), reqwest::Error> {


    let method_name:String = String::from("getblocksubsidy");
    let method_body:String;
	let height = height_supplied.unwrap_or(0);
	if height >= 0
	{
		method_body = String::from(format!("[{0}]",height));
	}
	else
	{
		method_body = String::from("[]");
	}
    let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
    println!("the body is{:?}",data );
    let result = komodorpcutil::request( some_user.clone(), data);
    return result;

}
/*
TODO getblocktemplate
Requires a JSON object plus a few optionals.
Need to check to understand this one.



*/



/*The get_local_solps method returns average local solutions per second since this node was started.

# Arguments
Name	    Type	            Description

# Response
Name	    Type     	        Description
"data" 	    (u32)	            the solutions-per-second average
*/ 

pub fn get_local_solps(
    some_user: komodorpcutil::KomodoRPC,)
     -> Result<(), reqwest::Error> {

    let method_name:String = String::from("getlocalsolps");
  	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
	let result = komodorpcutil::request(some_user.clone(), data);
    return result;
}

/*The getmininginfo method returns a json object containing mining-related information.

# Arguments
Name	    Type	            Description

# Response
Name	            Type     	        Description
"blocks"	        (numeric)	        the current block
"currentblocksize"	(numeric)	        the last block size
"currentblocktx"	(numeric)	        the last block transaction
"difficulty"	    (numeric)	        the current difficulty
"errors":		
"generate"	        (boolean)	        if the generation is on or off (see getgenerate or setgenerate calls)
"genproclimit"	    (numeric)	        the processor limit for generation; -1 if no generation (see getgenerate or setgenerate calls)
"localsolps"	    (numeric)	        the average local solution rate (solutions per second) since this node was started
"networksolps"	    (numeric)	        the estimated network solution rate (solutions per second)
"pooledtx":		
"testnet"	        (boolean)	        if using testnet or not
"chain"	            (string)	        the current network name as defined in BIP70 (main, test, regtest)
*/ 

pub fn get_mining_info(
    some_user: komodorpcutil::KomodoRPC,)
     -> Result<(), reqwest::Error> {

    let method_name:String = String::from("getmininginfo");
  	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
	let result = komodorpcutil::request(some_user.clone(), data);
    return result;
}


/*The getnetworksolps method returns the estimated network solutions per second based on the last n blocks.
Pass in blocks to override the default number of blocks. 
Use -1 to calculate according to the relevant difficulty averaging window. 
Pass in height to estimate the network speed at the time when a certain block was found.

# Arguments
Name	    Type	            Description
"blocks"	(u32, optional)	    the number of blocks   (Defaults to 120)
"height"    (int, optional)     the block height that corresponds to the requested data    (Defaults to -1)

# Response
Name	    Type     	        Description
"data"   	(u32)	            solutions per second, estimated
*/ 

pub fn get_network_solps(
    some_user: komodorpcutil::KomodoRPC,
    blocks_supplied: Option<u32>,
    height_supplied: Option<int>)
     -> Result<(), reqwest::Error> {


    let method_name:String = String::from("getnetworksolps");
    let method_body:String;
	let height = height_supplied.unwrap_or(0);
	if height >= 0
	{
		method_body = String::from(format!("[{0}]",height));
	}
	else
	{
		method_body = String::from("[]");
	}
    let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
    println!("the body is{:?}",data );
    let result = komodorpcutil::request( some_user.clone(), data);
    return result;

}