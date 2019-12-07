/*use super:: komodorpcutil;
use komodorpcutil::KomodoRPC;


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
    let method_name:String = String::from("getblockckhash");
    let method_body:String = String::from(format!("[{:?}]",index));
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
        method_body = String::from("[\"blockhash\": \"") + &temp_block_hash.to_string()+ &String::from(" \", \"nblocks\" : ") + &temp_nblocks.to_string() + &String::from("]");
    }
    else if (temp_nblocks <= 0) && !(temp_block_hash.is_empty())
    {
        method_body = String::from("[\"blockhash\": \"") + &temp_block_hash.to_string()+ &String::from(" \"]");
    }
    else if (temp_nblocks > 0) && (temp_block_hash.is_empty())
    {
        method_body = String::from("[\"nblocks\": ") + &temp_nblocks.to_string()+  &String::from("]");// &String::from(format!("[/"{0}/"]",temp_top));
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
    method_body = String::from("[\"txid\": \"") + &tx_id.to_string()+ &String::from(" \", \"index\" : ") + &index.to_string() + &String::from("]");
    let method_name:String = String::from("getchaintxstats");
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
        method_body = String::from("[\"txid\": \"") +
        &tx_id.to_string()+ &String::from(" \", \"vout\" : ") +
        &vout.to_string() + &String::from(" , \"includemempool\" : ") +
        &String::from("true") + &String::from("\" ]");

    }
    else
    {
        method_body = String::from("[\"txid\": \"") +
        &tx_id.to_string()+ &String::from(" \", \"vout\" : ") +
        &vout.to_string() + &String::from(" ]");

    }
    let method_name:String = String::from("getchaintxstats");
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
        method_body = String::from("[\"txid\": \"") +
        &tx_id.to_string()+ &String::from(" \", \"blockhash\" : ") +
        &block_hash.to_string() + &String::from(" ]");

    }
    else
    {

    }
    let method_name:String = String::from("getchaintxout");
    let data:String = String::from (komodorpcutil::generate_body(SomeUser.clone(),method_name,method_body));
    let result =komodorpcutil::request( SomeUser.clone(), data);
    return result;
}

*/
