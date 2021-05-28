#![allow(non_snake_case)]
use std::env;
use std::path::PathBuf;
use std::fs;
mod crypto;

fn main()
{
    // This is where pseudo-global flags will go
    let mut DEBUG_MODE = false;

    // Make sure the user's provided some CLI args
    if std::env::args().skip(1).len() == 0
    {
        eprintln! ("Usage:\n{} [options] <Command>", std::env::args().nth(0).unwrap_or("jrpm".to_string()));
        println! ("Use `{} --help` for a list of options and commands", std::env::args().nth(0).unwrap_or("jrpm".to_string()));
        std::process::exit(1);
    }

    // Parse the CLI args, and set some flags
    for arg in std::env::args().skip(1)
    {
        match &arg as &str
        {
            "-d" | "--debug" => DEBUG_MODE = true,
            _ => {}
        }
    }

    // Now get the last argument for the INTENT
    let INTENT = &std::env::args().last().unwrap() as &str;

    // Now that we have the CLI flags, let's do what we need to do with them
    if DEBUG_MODE { println! ("DEBUG: Debug mode activated"); }
   
    match INTENT
    {
        "generate-key" =>
        {
            if DEBUG_MODE
            {
                let keyFileContent = crypto::Encoder::generate_new_key();
                println! ("Key is {:?}", keyFileContent);
            }
            else { println! ("Debug mode must be enabled to use this feature."); }
        },
        "init" =>
        {
            let init_return = init();
            match init_return
            {
                0 => println! ("jrpm was succesfully initialized!"),
                1 => eprintln! ("Initialization aborted."),
                _ => eprintln! ("unrecognized return!")
            }
        },
        _ => {}
    }
}

fn init() -> i32
{
    // Check if the directory exists. If it doesn't, create it
    let mut dir_path = env::home_dir().expect("Home not found!");
    dir_path.push(".jrpm");
    if dir_path.is_dir() { println! ("{:?} already exists; continuing with initiation...", dir_path); }
    else { fs::create_dir_all(dir_path); }
    
    // Check if the key exists. If it does and the user wants to overwrite it, rename it to
    // .key.old and create a new key. If it doesn't exist, just make a new key
    let mut key_path = env::home_dir().expect("Home not found!");
    key_path.push(".jrpm");
    key_path.push(".key");
    if key_path.is_file()
    {
        println! ("IMPORTANT: YOU HAVE AN EXISTING KEY\nIf you choose to continue, it will be overwrited, making your passwords inaccesable.\nType \"cONFIRm\" to continue, type anything else to cancel.");
        let mut user_confirmation = String::new();
        let continue_pass = String::from("cONFIRm");
        let reader = std::io::stdin().read_line(&mut user_confirmation).unwrap_or(0);
        if user_confirmation.trim().ne(&continue_pass) { return 1; }
        else { println! ("You've selected to overwrite your key. Good luck with that."); }
    }
    else
    {
        let key_to_write = crypto::Encoder::generate_new_key();
        let key_as_string: String = key_to_write.into_iter().map(|i| i.to_string()).collect::<String>();
        fs::write(key_path, &key_as_string);
    }
    0
}

