mod utils;
mod models;
mod enums;
mod data;

use colored::*;
use std::env;
use crate::utils::analyze_project_call;

fn main() {
    let lazyp_ascii_art = r#"
 /$$                                     /$$$$$$$      |  LazyP is a CLI tool that will  help you organize
| $$                                    | $$__  $$     |  your  projects  better  ,  open them  faster and
| $$        /$$$$$$  /$$$$$$$$ /$$   /$$| $$  \ $$     |  be the lazy programmer/developer you usually are
| $$       |____  $$|____ /$$/| $$  | $$| $$$$$$$/     |
| $$        /$$$$$$$   /$$$$/ | $$  | $$| $$____/      |  Wanna keep whatching that  show/movie/video  but
| $$       /$$__  $$  /$$__/  | $$  | $$| $$           |  the only thing you see is the terminal  after  a
| $$$$$$$$|  $$$$$$$ /$$$$$$$$|  $$$$$$$| $$           |  very  long  time of coding ? LazyP will help you
|________/ \_______/|________/ \____  $$|__/           |
                               /$$  | $$               |  Left an uni/school excerise in half but  you are
            V-0.01BETA        |  $$$$$$/               |  too  lazy to open the tools you made  it  with ?
                               \______/                |  LazyP just did it !

"#.cyan();

    let instructions = r#"

lazyp           :
                |project| && |p|
                project                   -> This shows all the projects.
                project -o [PROJECT_NAME] -> This opens the project in it's prefered IDLE and tools
                project -a                -> This creates a project.
                project -e                -> This edits a project.
                project -d                -> This deletes a project.
                project -s                -> This saves a projects root directory directly based on
                                             the location you currently are.

                |watch| && |w|

                |uni| && |u|

"#;

    let args : Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{}", lazyp_ascii_art);
    }else {
        match args[1].as_str() {
            "-help" => println!("{}", instructions),
            "project" => analyze_project_call(args),
            _ => println!("🙍 No command found [{}], type -help to see all the commands", args[1].to_string())
        }
    }

}

