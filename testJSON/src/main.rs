

extern crate serde_json;
extern crate serde;

// create struct and derive struct from de/serialize traits
#[macro_use]
extern crate serde_derive;

// this will map json values to struct, used in commented out code
//use serde_json::Value as JsonValue;

// define new struct template for the json string
#[derive(Debug, Serialize, Deserialize)]
struct Person
{
    name: String,
    age: u8,
    has_hair: bool,
    address: Address,
    phone_numbers: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Address
{
    street: String,
    city: String,
    // i32 is signed 32 bit integer
    zip_code: i32
}


// https://github.com/V413H4V/Komodo-RPC-Library-Python
/*
	Rust has no class
	rpc_address		IP address of the node where the Komodo-daemon is running; Default: '127.0.0.1'
	rpc_port		Port number where the Komodo-daemon is listening for RPCs
	rpc_username	Username for RPC authentication
	rpc_password	Password for RPC authentication
	req_method		Request Method for RPCs; Default: 'POST'
	json_rpc_ver	Default:'1.0'
	rpc_id			ID for RPC requests Default:'curltest'
*/
#[derive(Debug, Serialize, Deserialize)]
struct KomodoRPC
{
    
    rpc_address: String,
    rpc_port: i32,
    rpc_username: String,
    rpc_password: String,
	req_method: String,
	json_rpc_ver: String,
    rpc_id: String
	
}



// 2 ways
// - commented out at the bottom - (simple) acces data through array like syntax
// - in main - map json data to a struct

pub fn main() {
    // https://github.com/serde-rs/json
    
    // raw string (r#) for json - object with 3 properties
    let json_str = r#"
        {
            "name": "asdf",
            "age": 123,
            "has_hair": true,
            "address":
            {
                "street": "some Street",
                "city": "some city",
                "zip_code": 1233
            },
            "phone_numbers":
            [
                "+44 1234567",
                "+44 2345678"
            ]
        }
    "#;
    /*
    // deserialize into json data struct - string to object
    let result = serde_json::from_str(json_str);
    println!("\njson result is {:?}\n", result);
    
    // deserialize into json data struct w/o checking values(use above) - string to object
    //let result2: Person = serde_json::from_str(json_str).unwrap();
    //println!("json result is {:?}\n", result2);
    
    // check for failure in json valid values
    if result.is_ok()
    {
        // CHANGED FROM JsonValue, maps values and assign to properties automatically
        let parsed: Person = result.unwrap();
        
        // convert to rust string?; '.as_str()'
        println!("The name is {}", parsed.name);
        println!("The age is {}", parsed.age);
        println!("The hair is {}", parsed.has_hair);
        println!("address street is {}", parsed.address.street);
        println!("address city is {}", parsed.address.city);
        println!("address zip is {}", parsed.address.zip_code);
        println!("phone number is {:?}", parsed.phone_numbers);
    }
    else
    {
        println!("\njson result is {:?}\n", result);
        println!("cant run");
    }
    
    */
    /*
    // https://doc.rust-lang.org/std/primitive.str.html
    // creating JSON by serializing data structure
    let somePerson = Person{
        name: "qwer".to_string(),
        age: 123,
        has_hair: true,
        address: Address{
            street: "a street".to_string(),
            city: "a city".to_string(),
            zip_code: 0987
        },
        phone_numbers: vec!["0867984".to_string(), "84766371".to_string()]
    };
    
    let jsonResult = serde_json::to_string(&somePerson);
    
    println!("{:?}", jsonResult);
    */
    
    
    
    
    /*
    // raw string (r#) for json - object with 3 properties
    let json_str = r#"
        {
            "name": "asdf",
            "age": 123,
            "has_hair": true
        }
    "#;

    // deserialize into data struct
    let result = serde_json::from_str(json_str);

    // check for failure in json valid values
    if result.is_ok()
    {
        // parse as json?
        let parsed: JsonValue = result.unwrap();
        
        // convert to rust string; '.as_str().upwrap()'
        println!("The name is {}", p["name"]);

    }
    else
    {

        println!("cant run");

    }*/
}