use std::process;


pub enum Command{
    GET {key:String},
    SET {key:String,value:String,expiry:Option<String>},
    DEL {key:String},
    EXIT,
    UNKNOWN
}
impl Command{

    pub fn parse_inputs(command:String)->Command{
    let parts= parse_command_line(command.clone());
    if parts.is_empty(){
        return Command::UNKNOWN
    }
    match parts[0].to_uppercase().as_str() {
       "GET" if parts.len()==2=>{
            Command::GET { key: parts[1].to_string() }
       },
       "SET" if parts.len()>=3=>{
            let key= parts[1].to_string();
            let value= parts[2].to_string();
            if parts.len()>3 && parts[3].to_uppercase()!="EX"{
                return Command::UNKNOWN
            }
            let expiry = if parts.len() == 5   {
    
               Some( String::from(parts[4].clone()))
            
                }
             else {
                None
            };

            Command::SET { key, value, expiry }
       },
       "DELETE" if parts.len()==2=>{
            Command::DEL { key: parts[1].to_string() }
       } ,
       "EXIT" if parts.len()==1=>{
            Command::EXIT
       }
       _=> Command::UNKNOWN
    }

    }

        

    pub fn handle_exit() {
        process::exit(0)
    }

    pub fn handle_unknown() {
        println!("Unknown Command");
    }
}

fn parse_command_line(input: String) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts.iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
}
