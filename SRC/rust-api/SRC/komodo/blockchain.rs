use super:: komodorpcutil;
use komodorpcutil::KomodoRPC;
use std::any::Any;



//getblockhashes
//gettxoutproof

//getblockchaininfo

pub fn coin_supply(
	some_user: komodorpcutil::KomodoRPC,
	height_supplied: Option<u32>)  
	-> Result<(), reqwest::Error> {
	/*
	The coinsupply method takes input of optional paramater called height
	that indicates the block height. If no height is provided, the function
	uses a default value of current blockchain's height.
	Function 
    @param height: (32 bit initeger, optional) the desired block height
    @return: JSON string containing:
        "result" (string) whether the request was successful
        "coin" (string) the ticker symbol of the coin for asset chains,
        "height" (integer) the height of this coin supply data
        "supply" (float) the transparent coin supply
        "zfunds" (float) the shielded coin supply (in zaddrs)
        "sprout" (float) the sprout coin supply (in zcaddrs)
        "total" (float) the total coin supply,
			i.e. sum of supply + zfunds
	*/
	let method_name:String = String::from("coinsupply");
	let method_body:String;
	
	let height = height_supplied.unwrap_or(0); //Default value is 0
	if height > 0 
	
	{
		method_body = String::from(format!("[\"{0}\"]",height));
	}
	else
	{
		method_body = String::from("[]");
	}
	print!("{}\n",method_body );
	let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
	print!("{}\n\n",data);
	let result =komodorpcutil::request( some_user.clone(), data);
    return result;
}

pub fn get_best_block_hash(
	some_user: komodorpcutil::KomodoRPC) 
	->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getbestblockhash");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
	let result =komodorpcutil::request( some_user.clone(), data);
    return result;

}

pub fn get_block(SomeUser: komodorpcutil::KomodoRPC, 
	height_or_hash: String,
	verbose: Option<bool>) ->Result<(), reqwest::Error>
{
let temp_verbose:bool = verbose.unwrap_or(false);
let method_name:String = String::from("getblock");
let method_body: String;
// if verbose is true

if temp_verbose
{
method_body = String::from("[\"")+
&height_or_hash +
&String::from("\", true]");
} 
else
{
method_body = String::from("[\"")+
&height_or_hash +
&String::from("\", false]");
}

let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
let result =komodorpcutil::request( SomeUser.clone(), data);
return result;

}




pub fn get_blockchain_info(
	some_user: komodorpcutil::KomodoRPC) 
->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getblockchaininfo");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
	let result =komodorpcutil::request( some_user.clone(), data);
    return result;

}


/*************      N		E		W			F	U	N	C	T	I	O	N	S			************/       

pub fn get_block_count(
	SomeUser: komodorpcutil::KomodoRPC) 
	->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getblockcount");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;

}

pub fn get_block_hash(
	SomeUser: komodorpcutil::KomodoRPC
	,index:u32) 
	->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getblockhash");
	let method_body:String = String::from(format!("[{:?}]",index));
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}
 


////////// A 		C 	O	N	F	U	S 	I	N	G 		F	U	N	C 	T	 I	O	N ********//////

pub fn get_block_hashes(
	SomeUser: komodorpcutil::KomodoRPC,
	high:u32, 
	low:u32,
	no_orphans:bool,
	logical_times:bool
)->Result<(), reqwest::Error>
{
	let method_body: String;
	let temp_no_orphans:String;
	let temp_logical_times:String;
	if no_orphans
	{
		temp_no_orphans = String::from("true");
	}
	else
	{
		temp_no_orphans = String::from("false");

	}

	if logical_times
	{
		temp_logical_times = String::from("true");
	}
	else
	{
		temp_logical_times = String::from("false");

	}


	let json_data_for_request = String::from (format!("{{\"noOrphans\":{0}, \"logicalTimes\":{1}}}",temp_no_orphans,temp_logical_times));
	/*method_body = String::from("[")
				+ &high.to_string() 
				+ &String::from(",")
				+ &low.to_string()
				+ &String::from(",\"")
				+ &json_data_for_request
				+ &String::from("\"]");

				print!("{}", method_body);
*/
//alternative
method_body = String::from("[")
+ &high.to_string() 
+ &String::from(",")
+ &low.to_string()
+ &String::from("]");

let method_name: String = String::from("getblockhashes");
let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
let result =komodorpcutil::request( SomeUser.clone(), data);
return result;
}



pub fn get_block_header(SomeUser: komodorpcutil::KomodoRPC, 
	hash: String,
	verbose: Option<bool>) ->Result<(), reqwest::Error>
{
	/*
Arguments
Name	Type	Description
"hash"	(string, required)	the block hash
verbose	(boolean, optional, default=true)	true returns a json object, false returns hex-encoded data
*/
let temp_verbose:bool = verbose.unwrap_or(true);
let method_name:String = String::from("getblockheader");
let method_body: String;
// if verbose is true

if temp_verbose
{
method_body = String::from("[\"")+
&hash +
&String::from("\", true]");
} 
else
{
method_body = String::from("[\"")+
&hash +
&String::from("\", false]");
}

let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
let result =komodorpcutil::request( SomeUser.clone(), data);
return result;

}

pub fn get_chain_tips(
	SomeUser: komodorpcutil::KomodoRPC) 
	->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getchaintips");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}

pub fn get_chain_tx_stats(
	SomeUser: komodorpcutil::KomodoRPC, 
	n_blocks:Option<u32>, 
	block_hash:Option<String>) 
	->Result<(), reqwest::Error>
{
	let method_body:String;
	let temp_nblocks = n_blocks.unwrap_or(0); //Default value is 0
	let temp_block_hash:String = block_hash.unwrap_or("".to_string());
    if (temp_nblocks > 0) && !(temp_block_hash.is_empty())
    {
		method_body = String::from("[\"blockhash\": \"") + 
		&temp_block_hash.to_string()+
		 &String::from(" \", \"nblocks\" : ") + 
		 &temp_nblocks.to_string() + 
		 &String::from("]");
    }
    else if (temp_nblocks <= 0) && !(temp_block_hash.is_empty())
    {
		method_body = String::from("[\"blockhash\": \"") + 
		&temp_block_hash.to_string()+ &String::from(" \"]");
	}
	else if (temp_nblocks > 0) && (temp_block_hash.is_empty())
    {
		method_body = String::from("[\"nblocks\": ") + 
		&temp_nblocks.to_string()+  &String::from("]");
	}
	else
	{
		method_body = String::from("[]");
	}
    let method_name:String = String::from("getchaintxstats");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}


pub fn get_difficulty(
	SomeUser: komodorpcutil::KomodoRPC) 
->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getdifficulty");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}


pub fn get_last_segid_stakes(
	SomeUser: komodorpcutil::KomodoRPC,
	depth:u32) 
	->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getlastsegidstakes");
	let method_body:String = String::from(format!("[{}]",depth));
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}


pub fn get_mempool_info(
	SomeUser: komodorpcutil::KomodoRPC) 
	->Result<(), reqwest::Error>
{
    let method_name:String = String::from("getmempoolinfo");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}

pub fn get_raw_mempool(
	SomeUser: komodorpcutil::KomodoRPC, 
	verbose:Option<bool>) 
	->Result<(), reqwest::Error>
{
	let temp_verbose:bool = verbose.unwrap_or(false);
	let method_name:String = String::from("getrawmempool");
	
	let method_body:String;
	if temp_verbose
	{
		method_body = String::from("[true]");
	}
	else
	{
		method_body = String::from("[false]");
	}
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}


pub fn get_spent_info(
	SomeUser: komodorpcutil::KomodoRPC, 
	tx_id:String, 
	index: u32) 
	->Result<(), reqwest::Error>
{
	let method_body:String;
	method_body = String::from("[{\"txid\": \"") + 
	&tx_id.to_string()+ &String::from(" \", \"index\" : ") + 
	&index.to_string() + &String::from("}]");
    let method_name:String = String::from("getspentinfo");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}

pub fn get_tx_out(
	SomeUser: komodorpcutil::KomodoRPC, 
	tx_id:String, 
	vout: u32, 
	include_mempool:Option<bool>) 
	->Result<(), reqwest::Error>
{
	/*
	Arguments
	Name	Type	Description
	"txid"	(string, required)	the transaction id
	vout	(numeric, required)	the vout value
	includemempool	(boolean, optional)	whether to include the mempool
	*/
	 let bool_include_mempool:bool = include_mempool.unwrap_or(false);
	 let method_body: String;
	if bool_include_mempool == true
	{
		method_body = String::from("[\"") + 
		&tx_id.to_string()+ 
		&String::from(" \", ") + 
		&vout.to_string() + 
		&String::from(",") + 
		&String::from("true") + 
		&String::from("]");

	}
	else
	{
		method_body = String::from("[\"") + 
		&tx_id.to_string()+ 
		&String::from(" \", ") + 
		&vout.to_string() + 
		&String::from(",") + 
		&String::from("false") + 
		&String::from("]");



	}
    let method_name:String = String::from("gettxout");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}

pub fn get_tx_out_proof(
	SomeUser: komodorpcutil::KomodoRPC, 
	tx_id:String, 
	block_hash:Option<String>) 
	->Result<(), reqwest::Error>
{
	/*
	Arguments
	Name	Type	Description
	"txid"	(string)	a transaction hash
	"blockhash"	(string, optional)	if specified, the method looks for the relevant transaction id in this block hash
	*/
	 let temp_block_hash:String = block_hash.unwrap_or("".to_string());
	 let method_body: String;
	 if (!temp_block_hash.is_empty())
	{
		method_body = String::from("[\"") + 
		&tx_id.to_string()+ 
		&String::from(" \", \" ") + 
		&temp_block_hash.to_string() +
		 &String::from(" \"]");

	}
	else 
	{
		method_body = String::from("[\"") + 
		&tx_id.to_string()+ 
		&String::from(" \"]");
	}
    let method_name:String = String::from("gettxoutproof");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}


pub fn get_tx_out_set_info(
	SomeUser: komodorpcutil::KomodoRPC) 
	->Result<(), reqwest::Error>
{
    let method_name:String = String::from("gettxoutsetinfo");
	let method_body:String = String::from("[]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}



pub fn kv_search(
	SomeUser: komodorpcutil::KomodoRPC,
	key:String) 
	->Result<(), reqwest::Error>
{
    let method_name:String = String::from("kvsearch");
	let method_body:String = String::from("[\"") 
	+ &key.to_string()+ 
	&String::from("\"]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}

pub fn kv_update(
	SomeUser: komodorpcutil::KomodoRPC,
	key:String,
	value: String,
	days: u32,
	pass_phrase: Option<String>) 
	->Result<(), reqwest::Error>
{
/*
Arguments
Name	Type	Description
"key"	(string, required)	key (should be unique)
"value"	(string, required)	value
"days"	(numeric, required)	amount of days before the key expires (1440 blocks/day); minimum 1 day
"passphrase"	(string, optional)	passphrase required to update this key
*/
//"params": ["examplekey", "examplevalue", "2", "examplepassphrase"]
    let method_name:String = String::from("kvupdate");
	let method_body:String;
	let temp_pass_phrase:String = pass_phrase.unwrap_or("".to_string());
	if ( temp_pass_phrase.is_empty())
	{
		method_body = String::from("[\"") +
		&key.to_string() + 
		&String::from("\", \"")+
		&value.to_string() +
		&String::from("\", \"")+
		&days.to_string()+
		&String::from("\"]");

	}
	else
	{
		method_body = String::from("[\"") +
		&key.to_string() + 
		&String::from("\", \"")+
		&value.to_string() +
		&String::from("\", \"")+
		&days.to_string()+
		&String::from("\", \"")+
		&temp_pass_phrase.to_string()+
		&String::from("\"]")

	}
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}

pub fn miner_ids(
	SomeUser: komodorpcutil::KomodoRPC
	,height:u32) 
	->Result<(), reqwest::Error>
{
/*
Arguments
Name	Type	Description
heights	(number)	the block height for the query
*/
    let method_name:String = String::from("minerids");
	let method_body:String = String::from(format!("[\"{:?}\"]",height));
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}


//????? Review this method again ??????
// what happens when two parameters are sent together
// Do we need to name th e paramater too?
pub fn notaries(
	SomeUser: komodorpcutil::KomodoRPC
	,height:Option<u32>,
	timestamp:Option<u32>) 
	->Result<(), reqwest::Error>
{
	let temp_height:u32 = height.unwrap_or(0);
	let temp_timestamp:u32 = timestamp.unwrap_or(0);

/*
Arguments
Name	Type	Description
height	(number)	the block height desired for the query
timestamp	(number)	the timestamp of the block desired for the query
*/
	let method_name:String = String::from("notaries");
	let method_body:String;
	if temp_height >= 0 && temp_timestamp >= 0
	{
		method_body = String::from("[\"")+
		&temp_height.to_string()+
		&String::from("\",\"")+
		&temp_timestamp.to_string()+
		&String::from("\"]");
	}
	else if temp_height >= 0
	{
		method_body = String::from("[\"")+
		&temp_height.to_string()+
		&String::from("\"]");
	}
	else if temp_timestamp >= 0
	{
		method_body = String::from("[\"")+
		&temp_timestamp.to_string()+
		&String::from("\"]");
	}
	else
	{
		//Todo Error handling here 
		print!("Error");
		method_body = String::from("");
	}
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}
 



//????? Review this method again ??????
// what happens when two parameters are sent together
// Do we need to name th e paramater too?
pub fn verify_chain(
	SomeUser: komodorpcutil::KomodoRPC
	,check_level:Option<u8>,
	num_blocks:Option<u16>) 
	->Result<(), reqwest::Error>
{
	let temp_check_level:u8 = check_level.unwrap_or(3);
	let temp_num_blocks:u16 = num_blocks.unwrap_or(288);

/*
The verifychain method verifies the coin daemon's blockchain database.

Depending on the state of your blockchain database and daemon, this call can take a prolonged period of time to complete.

#Arguments
Name	Type	Description
checklevel	(numeric, optional, 0-4, default=3)	indicates the thoroughness of block verification
numblocks	(numeric, optional, default=288, 0=all)	indicates the number of blocks to verify
*/
	let method_name:String = String::from("verifychain");
	let method_body = String::from("[")+
		&temp_check_level.to_string()+
		&String::from(",")+
		&temp_num_blocks.to_string()+
		&String::from("]");

	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}


pub fn verify_tx_out_proof(
	SomeUser: komodorpcutil::KomodoRPC,
	proof_string:String) 
	->Result<(), reqwest::Error>
{
    let method_name:String = String::from("verifytxoutproof");
	let method_body:String = String::from("[\"") 
	+ &proof_string.to_string()+ 
	&String::from("\"]");
	let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
	let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}






/*

  ____    _         ____     _____   _  __     _____   _    _              _____   _   _     _______   ______    _____   _______  
 |  _ \  | |       / __ \   / ____| | |/ /    / ____| | |  | |     /\     |_   _| | \ | |   |__   __| |  ____|  / ____| |__   __| 
 | |_) | | |      | |  | | | |      | ' /    | |      | |__| |    /  \      | |   |  \| |      | |    | |__    | (___      | |    
 |  _ <  | |      | |  | | | |      |  <     | |      |  __  |   / /\ \     | |   | . ` |      | |    |  __|    \___ \     | |    
 | |_) | | |____  | |__| | | |____  | . \    | |____  | |  | |  / ____ \   _| |_  | |\  |      | |    | |____   ____) |    | |    
 |____/  |______|  \____/   \_____| |_|\_\    \_____| |_|  |_| /_/    \_\ |_____| |_| \_|      |_|    |______| |_____/     |_|    
      _____               _____   ______    _____                                                                                 
     / ____|     /\      / ____| |  ____|  / ____|                                                                                
    | |         /  \    | (___   | |__    | (___                                                                                  
    | |        / /\ \    \___ \  |  __|    \___ \                                                                                 
    | |____   / ____ \   ____) | | |____   ____) |                                                                                
     \_____| /_/    \_\ |_____/  |______| |_____/                                                                                 
                                                   


*/


/*
	// TEST CASES FOR BLOCKCHAIN::COIN_SUPPLY ***
let test_coin_supply=  komodo::blockchain::coin_supply(someUser.clone(),None);
let test_coin_supply1=  komodo::blockchain::coin_supply(someUser.clone(),Some(194));
let test_coin_supply2=  komodo::blockchain::coin_supply(someUser.clone(),Some(111));

	// TEST CASES FOR blockchain::get_best_blockhash **
let test_getbestblockhash =  komodo::blockchain::get_best_block_hash(someUser.clone());


	// TEST CASES FOR blockchain::get_block **

let  test_get_bock1 = komodo::blockchain::get_block(someUser.clone(),"00e61fc943e54ccbfd776cfae344fef96033932fc0c60f3e64565bd1b7b34d3e".to_string(),Some(true) );
let  test_get_bock2 = komodo::blockchain::get_block(someUser.clone(),10.to_string(),Some(true) );
let  test_get_bock3= komodo::blockchain::get_block(someUser.clone(),"00e61fc943e54ccbfd776cfae344fef96033932fc0c60f3e64565bd1b7b34d3e".to_string(),None );
let  test_get_bock4 = komodo::blockchain::get_block(someUser.clone(),"00e61fc943e54ccbfd776cfae344fef96033932fc0c60f3e64565bd1b7b34d3e".to_string(),Some(false) );

	// TEST CASES FOR blockchain::get_blockchain_info **

let test_blockchain_info= komodo::blockchain::get_blockchain_info(someUser.clone());

	// TEST CASES FOR blockchain::get_block_count **

let test_get_block_count= komodo::blockchain::get_block_count(someUser.clone);

	// TEST CASES FOR blockchain::get_block_hash **

let test_get_block_hash1= komodo::blockchain::get_block_hash(someUser.clone(),0);
let test_get_block_hash1= komodo::blockchain::get_block_hash(someUser.clone(),131);

// TEST CASES FOR blockchain::get_block_hash **
let test_get_block_hash1= komodo::blockchain::get_block_hashes(someUser.clone(),1531614698, 1531614498,true,false);


// TEST CASES FOR blockchain::get_block_header **

let  test_get_bock_header = komodo::blockchain::get_block(someUser.clone(),"00e61fc943e54ccbfd776cfae344fef96033932fc0c60f3e64565bd1b7b34d3e".to_string(),Some(false) );
let  test_get_bock_header1 = komodo::blockchain::get_block(someUser.clone(),"00e61fc943e54ccbfd776cfae344fef96033932fc0c60f3e64565bd1b7b34d3e".to_string(),None );

// TEST CASES FOR blockchain::get_chain_tips**

let test_get_chain_tips = komodo::blockchain::get_block_hash(someUser.clone());

//TEST CASES FOR blockchain::get_chain_tx_stats **
 S T I L L 		N O T 		S U R E ??? HOW TO PASS PARAMATERS? NO EXAMPLE GIVEN 
let test_get_chain_tx_stats = komodo::blockchain::get_chain_tx_stats(someUser.clone(),Some(10),Some("00e61fc943e54ccbfd776cfae344fef96033932fc0c60f3e64565bd1b7b34d3e".to_string()));
let test_get_chain_tx_stats1 = komodo::blockchain::get_chain_tx_stats(someUser.clone(),None,Some("00e61fc943e54ccbfd776cfae344fef96033932fc0c60f3e64565bd1b7b34d3e".to_string()));
let test_get_chain_tx_stats2 = komodo::blockchain::get_chain_tx_stats(someUser.clone(),Some(10),None);
let test_get_chain_tx_stats3 = komodo::blockchain::get_chain_tx_stats(someUser.clone(),None, None);

//TEST CASES FOR blockchain::get_difficulty **

 let test_get_difficulty = komodo::blockchain::get_difficulty(someUser.clone());



 //TEST CASES FOR blockchain::get_last_segid_stakes **

let test_get_last_segid_stakes = komodo::blockchain::get_last_segid_stakes(someUser.clone(),1);
 


 //TEST CASES FOR blockchain::get_mempool_info **
 let test_get_mempool_info = komodo::blockchain::get_mempool_info(someUser.clone());

 
 //TEST CASES FOR blockchain::get_raw_mempool  **
 let test_get_raw_mempool= komodo::blockchain::get_raw_mempool(someUser.clone(),Some(true));
let test_get_raw_mempool1= komodo::blockchain::get_raw_mempool(someUser.clone(),Some(false));
let test_get_raw_mempool= komodo::blockchain::get_raw_mempool(someUser.clone(),None);

 //TEST CASES FOR blockchain::get_spent_info **
let test_get_spent_info= komodo::blockchain::get_spent_info(someUser.clone(),"4479f2c05ba22adf2333db724f247a09effcc9edea8c079da0da05d3a0451064".to_string(),0);


//TEST CASES FOR blockchain::get_tx_out **
let test_get_tx_out= komodo::blockchain::get_tx_out(someUser.clone(),"d7f9b34ad3e86f48fce55dbec1f2925b8dcbd15199977fd75d224254db9ae80d".to_string(),0,None);
let test_get_tx_out1= komodo::blockchain::get_tx_out(someUser.clone(),"d7f9b34ad3e86f48fce55dbec1f2925b8dcbd15199977fd75d224254db9ae80d".to_string(),0,Some(true));


//TEST CASES FOR blockchain::get_tx_out_proof **

let test_get_tx_out_proof= komodo::blockchain::get_tx_out_proof(someUser.clone(),"d7f9b34ad3e86f48fce55dbec1f2925b8dcbd15199977fd75d224254db9ae80d".to_string(),Some("hello".to_string()));
let test_get_tx_out_proof1= komodo::blockchain::get_tx_out_proof(someUser.clone(),"d7f9b34ad3e86f48fce55dbec1f2925b8dcbd15199977fd75d224254db9ae80d".to_string(),None);


//TEST CASES FOR blockchain::get_tx_out_set_info **
let test_get_tx_out_set_info= komodo::blockchain::get_tx_out_set_info(someUser.clone());

//TEST CASES FOR blockchain::kv_search **
let test_kv_search= komodo::blockchain::kv_search(someUser.clone(),"Hello".to_string());


//TEST CASES FOR blockchain::kv_update **
let test_kv_update= komodo::blockchain::kv_update(someUser.clone(),"Hello".to_string(),"hi".to_string(),10,Some("hi".to_string()));
let test_kv_update= komodo::blockchain::kv_update(someUser.clone(),"Hello".to_string(),"hi".to_string(),10,None);

////TEST CASES FOR blockchain::miner_ids**
let test_miner_ids= komodo::blockchain::miner_ids(someUser.clone(),10);


////TEST CASES FOR blockchain::notaries**
let test_notaries= komodo::blockchain::notaries(someUser.clone(),Some(10),Some(100));
let test_notaries= komodo::blockchain::notaries(someUser.clone(),None,Some(100));
let test_notaries= komodo::blockchain::notaries(someUser.clone(),Some(10),None);
let test_notaries= komodo::blockchain::notaries(someUser.clone(),None, None);

////TEST CASES FOR blockchain::verify_chain**
let test_verify_chain= komodo::blockchain::verify_chain(someUser.clone(),Some(0),Some(100));
let test_verify_chain= komodo::blockchain::verify_chain(someUser.clone(),None,Some(100));
let test_verify_chain= komodo::blockchain::verify_chain(someUser.clone(),Some(10),None);
let test_verify_chain= komodo::blockchain::verify_chain(someUser.clone(),None, None);

////TEST CASES FOR blockchain::verify_tx_out_proof

let test_verify_tx_out_proof= komodo::blockchain::verify_tx_out_proof(someUser.clone(),"040000004cd8bd98c66020496d0b34a5f5412400146ba10d6c7ab4286f84f7008d8d390e9ca9575183f60906e293e9766997396bec59f1c0b966085de3d17f8ac3c9d5280000000000000000000000000000000000000000000000000000000000000000da05975bf50e0f202d004b81fcc388cfd411d8c7c59a548e070b5affe938ce8ce830f10b298b00002402939a9a31df1305b40d26d9748283b102c708258717248d0d63f01d2957d8e3dcf56f6e03000000022e4babc29707fbdd8da2e4277b7c8b8b09e837f409eb047c936904d75fc8e6267a9dcf39838a70d552bf5e246116bee43e93178916aae388d5bd87bf2e4a1fc7010d".to_string());



*/
