/*
*
* This is where I put my structs, In the future this may be directory its own
* @Author : Lampros karachristos
*
*/

use std::env;
use std::io::Write;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::data;
use crate::models::Project;
use crate::enums::Editor::*;
use crate::data::*;
use std::process::{Command, Stdio};

// Opening projects functions

pub fn analyze_project_call(args: Vec<String>) {
    if args.len() == 2 && args[1].as_str() == "project" {
        show_projects();
    } else {
        match args[2].as_str() {
            "-o" => open_project(get_project_by_id(args[3].as_str()).unwrap()),
            "-a" => create_project(false, ""),
            "-e" => edit_project(args[3].as_str()),
            "-d" => delete_project(args[3].as_str()),
            "-s" => create_project(true, args[3].as_str()),
            _ => println!("🙍 Nothing here, type -help for instructions")
        }
    }
}

pub fn create_project(is_working_dir: bool, x: &str) {
    let mut title = String::new();
    let mut id = String::new();
    let mut working_dir = String::new();
    let mut editor = VsCode; // Assuming VsCode as default editor

    let idle_options = &[
        IntelijUlt,
        IntelijCom,
        RustRover,
        VsCode,
        Nvim,
        Fleet,
        WebStorm,
        PlainText,
    ];

    print!("🔥 Name of the project >>>");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut title)
        .expect("❌ Failed to read line!");

    print!("📜 ID of the project (unique and small for your convenience) >>>");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut id)
        .expect("❌ Failed to read line!");

    if is_working_dir {
        // Get the directory of the currently executing binary
        working_dir = x.to_string();
    } else {
        print!("📦 Working dir of the project >>> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut working_dir)
            .expect("❌ Failed to read line!");
    }

    print!("🗿 Select editor >>>");
    std::io::stdout().flush().unwrap();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(idle_options)
        .interact();

    match selection {
        Ok(index) => {
            editor = idle_options[index].clone();
        }
        Err(err) => {
            println!("❌ Error: {}", err);
            return;
        }
    }

    let id = id.trim().parse().expect("❌ Could not parse String");
    let title = title.trim().parse().expect("❌ Could not parse String");
    let working_dir = working_dir.trim().parse().expect("❌ Could not parse String");

    // Create the project
    let project = Project {
        id,
        title,
        working_dir,
        editor,
    };

    add_project(project);

    println!("🎉 Project created successfully 🎉");
}

pub fn edit_project(project_id: &str) {
    let mut title = String::new();
    let mut working_dir = String::new();
    let mut editor = VsCode; // Assuming VsCode as default editor

    let idle_options = &[
        IntelijUlt,
        IntelijCom,
        RustRover,
        VsCode,
        Nvim,
        Fleet,
        WebStorm,
        PlainText,
    ];

    print!("🔥 Name of the project >>>");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line!");

    // Get the directory of the currently executing binary
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            working_dir = exe_dir.to_string_lossy().into_owned();
        } else {
            println!("❌ Failed to get parent directory of the executable path!");
            return;
        }
    } else {
        println!("❌ Failed to get current executable path!");
        return;
    }

    print!("🗿 Select editor >>>");
    std::io::stdout().flush().unwrap();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(idle_options)
        .interact();

    match selection {
        Ok(index) => {
            editor = idle_options[index].clone();
        }
        Err(err) => {
            println!("❌ Error: {}", err);
            return;
        }
    }

    let title = title.trim().parse().expect("[!] Could not parse String");
    let working_dir = working_dir.trim().parse().expect("[!] Could not parse String");

    // Create the project
    let project = Project {
        id: project_id.to_string(),
        title,
        working_dir,
        editor,
    };

    data::edit_project(project);

    println!("🎉 Project edited successfully 🎉");
}

pub fn show_projects() {
    let projects = get_projects();

    println!("======= Projects =======");
    for project in projects.projects {
        println!("📂 ID: {} ||📝 Title: {} ||✏️ Editor: {} ",
                 project.id, project.title, project.editor);
    }
    println!("======= That's all  =======");
}

pub fn open_project(project: Project) {
    match project.editor {
        IntelijUlt => open_with_intelij_ult(project),
        IntelijCom => open_with_intelij_com(project),
        RustRover => open_with_intelij_rover(project),
        VsCode => open_with_intelij_code(project),
        Fleet => open_with_intelij_fleet(project),
        Nvim => open_with_intelij_vim(project),
        WebStorm => open_with_intelij_storm(project),
        PlainText => open_with_intelij_text(project)
    }
}

fn open_with_intelij_ult(project: Project) {
    execute_editor("/home/lampros/.local/share/JetBrains/Toolbox/apps/intellij-idea-ultimate/bin/idea.sh"
                   , project.working_dir.as_str());
}

fn open_with_intelij_com(project: Project) {
    execute_editor("/home/lampros/.local/share/JetBrains/Toolbox/apps/intellij-idea-community-edition/bin/idea.sh"
                   , project.working_dir.as_str());
}

fn open_with_intelij_rover(project: Project) {
    execute_editor("/home/lampros/.local/share/JetBrains/Toolbox/apps/rustrover/bin/rustrover.sh"
                   , project.working_dir.as_str());
}

fn open_with_intelij_code(project: Project) {
    execute_editor("code"
                   , project.working_dir.as_str());
}

fn open_with_intelij_vim(project: Project) {
    execute_editor("nvim"
                   , project.working_dir.as_str());
}

fn open_with_intelij_fleet(project: Project) {
    execute_editor("/home/lampros/.local/share/JetBrains/Toolbox/apps/fleet/bin/fleet"
                   , project.working_dir.as_str());
}

fn open_with_intelij_storm(project: Project) {
    execute_editor("/home/lampros/.local/share/JetBrains/Toolbox/apps/webstorm/bin/webstorm.sh"
                   , project.working_dir.as_str());
}

fn open_with_intelij_text(project: Project) {
    execute_editor("nano"
                   , project.working_dir.as_str());
}

fn execute_editor(editor_loc: &str, dir_loc: &str) {
    // Create a new Command for executing the Bash command
    //println!("{} {}", editor_loc, dir_loc);
    let mut cmd = Command::new(editor_loc);

    // Add the directory location as an argument
    cmd.arg(dir_loc);

    // Configure the command to use /dev/null for stdin, stdout, and stderr
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    // Spawn the command asynchronously
    match cmd.spawn() {
        Ok(_child) => {
            // Command spawned successfully
            println!("💻 Editor is starting... Wait !");
        }
        Err(e) => {
            eprintln!("❌ Failed to execute command: {}", e);
        }
    }
}
