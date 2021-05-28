#![allow(non_snake_case)]
use std::env;
use std::path::PathBuf;
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
        _ => {}
    }
}

