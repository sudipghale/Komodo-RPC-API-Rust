use super:: komodorpcutil;
use komodorpcutil::KomodoRPC;

pub fn decode_raw_transaction(
    some_user: komodorpcutil::KomodoRPC,
    hex: String)
     -> Result<(), reqwest::Error> {
        let method_name:String = String::from("decoderawtransaction");
        let method_body:String;
        method_body = String::from("[\"hex\": \"") + &hex.to_string() + &String::from("]");
        let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
        println!("the body is{:?}",data );
        let result = komodorpcutil::request( some_user.clone(), data);
        return result;
    }

pub fn decode_script(
   some_user: komodorpcutil::KomodoRPC,
    hex: String)
     -> Result<(), reqwest::Error> {
        let method_name:String = String::from("decodescript");
        let method_body:String;
        method_body = String::from("[\"hex\": \"") + &hex.to_string() + &String::from("]");
        let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
        println!("the body is{:?}",data );
        let result = komodorpcutil::request( some_user.clone(), data);
        return result;
    }

pub fn fund_raw_transaction(
    some_user: komodorpcutil::KomodoRPC,
    hexstring: String)
     -> Result<(), reqwest::Error> {
        let method_name:String = String::from("fundrawtransaction");
        let method_body:String;
        method_body = String::from("[\"hexstring\": \"") + &hexstring.to_string() + &String::from("]");
        let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
        println!("the body is{:?}",data );
        let result = komodorpcutil::request( some_user.clone(), data);
        return result;
    }
        
pub fn get_raw_transaction(
    some_user: komodorpcutil::KomodoRPC,
    txid: String,
    verbose_supplied: Option<u32>)
     -> Result<(), reqwest::Error> {
        let method_name:String = String::from("getrawtransaction");
        let method_body:String;
        let verbose = verbose_supplied.unwrap_or(0);
        if(verbose == 0)
        {
            method_body = String::from("[\"txid\": \"") + &txid.to_string() + &String::from(" \", \"verbose\" : ") + &verbose.to_string() + &String::from("]");
        }
        else
        {
            method_body = String::from("[\"txid\": \"") + &txid.to_string() + &String::from("]");
        }
        let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
        println!("the body is{:?}",data );
        let result = komodorpcutil::request( some_user.clone(), data);
        return result;
    }

pub fn send_raw_transaction(
    some_user: komodorpcutil::KomodoRPC,
    hexstring: String,
    allow_high_fees_supplied: Option<bool>)
     -> Result<(), reqwest::Error> {
        let method_name:String = String::from("getrawtransaction");
        let method_body:String;
        let allow_high_fees = allow_high_fees_supplied.unwrap_or(false);
        method_body = String::from("[\"hexstring\": \"") + &hexstring.to_string() + &String::from(" \", \"allow_high_fees\" : ") + &allow_high_fees.to_string() + &String::from("]");
        let data:String = String::from (komodorpcutil::generate_body(some_user.clone(),method_name,method_body));
        let result = komodorpcutil::request( some_user.clone(), data);
        return result;
     }
    