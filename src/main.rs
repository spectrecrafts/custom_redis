use std::{collections::HashMap, io};
use std::process;
// #[derive(Debug)]

// struct Redis{
//     id:String,
//     key:String,
//     value:String
// }

// enum Command{
//     get,
//     set
// }


fn main(){
    let mut store: HashMap<String,String>= HashMap::new();
   loop {

    
    let mut command= String::new();

    if command=="exit"{
            process::exit(0)
    }
    
    io::stdin().read_line(&mut command).expect("Failed to read line"); 
    let command_parts: Vec<String>=command.trim().split_whitespace()
.map(String::from).collect();

    if (command_parts[0].to_lowercase()=="set"&&command_parts.len()<3)||(command_parts[0].to_lowercase()=="get" && command_parts.len()!=2){
        println!("Invalid format");
        continue;
    }


    match command_parts[0].to_lowercase().as_str()
{
"get"=>{
match store.get(&command_parts[1]){
   Some(val)=>println!("{}",val),
    None=>println!("The value does not exist")
}


},
    "set"=>{store.insert(command_parts[1].clone(), command_parts[2].clone());println!("OK")},
    _other=>println!("Invalid command")
}
   }

}