use std::env;
use std::fs;

fn main() {
    let args: Vec<String>       =   env::args().collect();
    
    let config          =   Config::build(&args).unwrap_or_else(|err|{
        print!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
                                      
    run(config).unwrap_or_else(|err|{
        print!("Application error: {}", err);
        std::process::exit(1);
    });
    
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!    ("Searching for '{}' in file '{}'", config.query, config.filename);
    
    let contents        =   fs::read_to_string(config.filename)?;
                                    

    print!      ("With text:\n{}", contents); 

    Ok(())
}

struct Config {
    query:      String,
    filename:   String,
}

impl Config {
    
    // fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         panic!("Not enough arguments");
    //     }

    //     let query       =   args[1].clone();
    //     let filename    =   args[2].clone();

    //     Config { query, filename }
    // }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query       =   args[1].clone();
        let filename    =   args[2].clone();

        Ok(Config { query, filename })
    }
    
}
