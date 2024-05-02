
/*
*
* This is where i put my data handling functions, In the future this may be directory its own
* @Author : Lampros karachristos
*
*/

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use crate::models::{Project, Projects};

// PROJECT HANDLE FUNCTION

pub fn get_projects() -> Projects{

    let mut projects_file = File::open("data/projects.json")
        .expect("[!] Could not read the projects file");
    let mut projects_json = "".to_string();
    projects_file.read_to_string(&mut projects_json)
        .unwrap();
        //.expect("[!] Could not parse data from projects file");

    // It returns project
    serde_json::from_str(&projects_json)
        .expect("[!] Could not deserialize content of Projects file")
}

pub fn add_project(new_project: Project, mut projects: Projects) -> Projects {

    projects.projects.push(new_project);

    let mut projects_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data/projects.json")
        .expect("[!] Could not open projects file for writing");

    let updated_projects_json = serde_json::to_string(&projects)
        .expect("[!] Could not serialize projects to JSON");

    projects_file.write_all(updated_projects_json.as_bytes())
        .expect("[!] Could not write projects to file");

    // Returns the new projects list
    projects
}

pub fn edit_project(edited_project: Project, mut projects: Projects) -> Projects {
    // Find the index of the project to edit
    if let Some(index) = projects.projects.iter().position(|p| p.id == edited_project.id) {
        // Update the project at the found index
        projects.projects[index] = edited_project;

        // Write the updated projects back to the file
        let mut projects_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("data/projects.json")
            .expect("[!] Could not open projects file for writing");

        let updated_projects_json = serde_json::to_string(&projects)
            .expect("[!] Could not serialize projects to JSON");

        projects_file.write_all(updated_projects_json.as_bytes())
            .expect("[!] Could not write projects to file");
    }

    projects
}

pub fn delete_project(project_id: String, mut projects: Projects) -> Projects {
    // Find the index of the project to delete
    if let Some(index) = projects.projects.iter().position(|p| p.id == project_id) {
        // Remove the project at the found index
        projects.projects.remove(index);

        // Write the updated projects back to the file
        let mut projects_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("data/projects.json")
            .expect("[!] Could not open projects file for writing");

        let updated_projects_json = serde_json::to_string(&projects)
            .expect("[!] Could not serialize projects to JSON");

        projects_file.write_all(updated_projects_json.as_bytes())
            .expect("[!] Could not write projects to file");
    }

    projects
}