mod utils;
mod models;
mod enums;
mod data;

use colored::*;
use std::env;
use crate::utils::analyze_project_call;

fn main() {
    let lampros_codes_ascii_art = r#"
 /$$                                                                 /$$$$$$                /$$                  
| $$                                                                /$$__  $$              | $$                  
| $$       /$$$$$$ /$$$$$$/$$$$  /$$$$$$  /$$$$$$  /$$$$$$  /$$$$$$| $$  \__/ /$$$$$$  /$$$$$$$ /$$$$$$  /$$$$$$$
| $$      |____  $| $$_  $$_  $$/$$__  $$/$$__  $$/$$__  $$/$$_____| $$      /$$__  $$/$$__  $$/$$__  $$/$$_____/
| $$       /$$$$$$| $$ \ $$ \ $| $$  \ $| $$  \__| $$  \ $|  $$$$$$| $$     | $$  \ $| $$  | $| $$$$$$$|  $$$$$$ 
| $$      /$$__  $| $$ | $$ | $| $$  | $| $$     | $$  | $$\____  $| $$    $| $$  | $| $$  | $| $$_____/\____  $$
| $$$$$$$|  $$$$$$| $$ | $$ | $| $$$$$$$| $$     |  $$$$$$//$$$$$$$|  $$$$$$|  $$$$$$|  $$$$$$|  $$$$$$$/$$$$$$$/
|________/\_______|__/ |__/ |__| $$____/|__/      \______/|_______/ \______/ \______/ \_______/\_______|_______/ 
                               | $$  ~~ Your not-that-smart-bean on the drawer ~~
            v-0.1BETA          | $$                                               ~~ now on a CLI tool ~~
                               |__/
"#.cyan();


    let welcome = r#"

[*] lampros-shell is a training project for rust that is going to help me better
understand the procces of creating a CLI application/tooll in my case this is more like a tool

[*] Thing lampros-shell is going to help you :
                                              1) Fast project and tool opening
                                              2) Fast Netflix, movies joy, youtube opening
                                              3) University tools and apps opening
                                              Thats all !

[*] Type -help to see all the instructions
"#.green();

    let instructions = r#"

lampros-shell :
                |project|
                project                   -> This shows all the projects.
                project -o [PROJECT_NAME] -> This opens the project in it's prefered IDLE and tools
                project -a                -> This creates a project.
                project -e                -> This edits a project.
                project -d                -> This deletes a project.
                project -s                -> This saves a projects root directory directly based on
                                             the location you currently are.

                |watch|

                |uni|

"#;

    let args : Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{}", lampros_codes_ascii_art);
        println!("{}", welcome);
    }else {
        match args[1].as_str() {
            "-help" => println!("{}", instructions),
            "project" => analyze_project_call(args),
            _ => println!("Nothing found!")
        }
    }

}

