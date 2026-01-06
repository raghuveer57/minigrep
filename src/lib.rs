use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!    ("Searching for '{}' in file '{}'", config.query, config.filename);
    
    let contents        =   fs::read_to_string(config.filename)?;
                                    

    print!      ("With text:\n{}", contents); 

    Ok(())
}

pub struct Config {
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

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query       =   args[1].clone();
        let filename    =   args[2].clone();

        Ok(Config { query, filename })
    }
    
}