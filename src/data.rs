
/*
*
* This is where I put my data_files handling functions, In the future this may be directory its own
* @Author : Lampros karachristos
*
*/

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use crate::models::{Project, Projects};

// PROJECT HANDLE FUNCTION

fn get_projects_file_path() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().expect("❌ Failed to get home directory");
    home_dir.join(".config").join("lazyp").join("data").join("projects.json")
}

pub fn get_projects() -> Projects{
    let projects_file_path = get_projects_file_path();
    // Open projects.json file
    let mut projects_file = File::open(&projects_file_path)
        .expect("❌ Could not read the projects file");
    let mut projects_json = String::new();
    projects_file.read_to_string(&mut projects_json)
        .expect("❌ Could not read projects file");

    // Deserialize projects from JSON
    serde_json::from_str(&projects_json)
        .expect("❌ Could not deserialize content of Projects file")
}

pub fn get_project_by_id(project_id: &str) -> Option<Project> {
    // Open and read the JSON file
    let projects = get_projects();

    // Find the project with the matching ID
    let project = projects.projects.into_iter()
        .find(|p| p.id == project_id)
        .map(|p| p.clone());

    project
}

pub fn add_project(new_project: Project) -> Projects {
    let mut projects = get_projects();
    projects.projects.push(new_project);

    let projects_file_path = get_projects_file_path();
    let mut projects_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&projects_file_path)
        .expect("❌ Could not open projects file for writing");

    let updated_projects_json = serde_json::to_string(&projects)
        .expect("❌ Could not serialize projects to JSON");

    projects_file.write_all(updated_projects_json.as_bytes())
        .expect("❌ Could not write projects to file");

    projects
}

pub fn edit_project(edited_project: Project) -> Projects {
    let mut projects = get_projects();

    if let Some(index) = projects.projects.iter().position(|p| p.id == edited_project.id) {
        projects.projects[index] = edited_project;

        let projects_file_path = get_projects_file_path();
        let mut projects_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&projects_file_path)
            .expect("❌ Could not open projects file for writing");

        let updated_projects_json = serde_json::to_string(&projects)
            .expect("❌ Could not serialize projects to JSON");

        projects_file.write_all(updated_projects_json.as_bytes())
            .expect("❌ Could not write projects to file");
    }

    projects
}

pub fn delete_project(project_id: &str){
    let mut projects = get_projects();

    if let Some(index) = projects.projects.iter().position(|p| p.id == project_id) {
        projects.projects.remove(index);

        let projects_file_path = get_projects_file_path();
        let mut projects_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&projects_file_path)
            .expect("❌ Could not open projects file for writing");

        let updated_projects_json = serde_json::to_string(&projects)
            .expect("❌ Could not serialize projects to JSON");

        projects_file.write_all(updated_projects_json.as_bytes())
            .expect("❌ Could not write projects to file");
    }

    println!("🙅 Project deleted successfully")
}
