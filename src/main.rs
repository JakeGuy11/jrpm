#![allow(non_snake_case)]
use std::env;
use std::path::PathBuf;
use std::fs;
use home;
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
        "add-site" =>
        {
            if DEBUG_MODE
            {
                let site_return = add_site();
                match site_return
                {
                    0 => println! ("Your password was sucessfully saved!"),
                    1 => println! ("Password creation aborted"),
                    _ => eprintln! ("unrecognized return!")
                }
            }
            else { println! ("Debug mode must be enabled to use this feature."); }
        },
        "init" =>
        {
            let init_return = init();
            match init_return
            {
                0 => println! ("JPRM was succesfully initialized!"),
                1 => eprintln! ("Initialization aborted."),
                _ => eprintln! ("unrecognized return!")
            }
        },
        _ => {}
    }
}

fn add_site() -> i32
{
    // Prompt the user for the site they would like to save for
    println! ("=============================================");
    println! ("Passwords are stored by JPRM using a keyword;");
    println! ("It is suggested that you use a common keyword");
    println! ("so you don't forget, such as `gmail` for a");
    println! ("gmail account.");
    println! ("Enter the keyword you would like to associate");
    println! ("with this password:");
    let mut site_keyword = String::new();
    std::io::stdin().read_line(&mut site_keyword).unwrap_or(0);
    println! ("=============================================");
    let file_name: String = site_keyword.trim().to_owned() + ".jrpm";
    let mut file_full_path = home::home_dir().expect("Home not found!");
    file_full_path.push(".jrpm");
    file_full_path.push(&file_name);
    if file_full_path.is_file()
    {
        println! ("IMPORTANT: you already have a password saved under that keyword. Would you like to overwrite it? [y/N]");
        let mut user_confirmation = String::new();
        let continue_pass = String::from("y");
        std::io::stdin().read_line(&mut user_confirmation).unwrap();
        if user_confirmation.trim().ne(&continue_pass) { return 1; }
        else { println! ("You've selected to overwrite your password."); }
    }
    0
}

fn init() -> i32
{
    // Check if the directory exists. If it doesn't, create it
    let mut dir_path = home::home_dir().expect("Home not found!");
    dir_path.push(".jrpm");
    if dir_path.is_dir() { println! ("{:?} already exists; continuing with initiation...", dir_path); }
    else { if let Err(e) = fs::create_dir_all(dir_path) { eprintln! ("Failed to create directory: {:?}", e); } }
    
    // Check if the key exists. If it does and the user wants to overwrite it, rename it to
    // .key.old and create a new key. If it doesn't exist, just make a new key
    let mut key_path = home::home_dir().expect("Home not found!");
    key_path.push(".jrpm");
    key_path.push(".key");
    if key_path.is_file()
    {
        println! ("IMPORTANT: YOU HAVE AN EXISTING KEY\nIf you choose to continue, it will be overwrited, making your passwords inaccesable.\nType \"cONFIRm\" to continue, type anything else to cancel.");
        let mut user_confirmation = String::new();
        let continue_pass = String::from("cONFIRm");
        std::io::stdin().read_line(&mut user_confirmation).unwrap_or(0);
        if user_confirmation.trim().ne(&continue_pass) { return 1; }
        else { println! ("You've selected to overwrite your key. Good luck with that."); }
        let mut new_key_path = key_path.clone();
        new_key_path.pop();
        new_key_path.push(".key.old");
        if let Err(e) = fs::rename(&key_path, new_key_path) { eprintln! ("Failed to move .key to .key.old: {:?}", e); }
    }
    let key_to_write = crypto::Encoder::generate_new_key();
    let key_as_string: String = key_to_write.into_iter().map(|i| i.to_string()).collect::<String>();
    if let Err(e) = fs::write(key_path, &key_as_string) { eprintln! ("Failed to write to file: {:?}", e); }
    0
}

