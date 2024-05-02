/*
*
* This is where I put my structs, In the future this may be directory its own
* @Author : Lampros karachristos
*
*/

use serde::{Deserialize, Serialize};
use crate::enums::Editor;

#[derive(Debug, Serialize, Deserialize, Clone,)]
pub struct Project {
    pub id : String,
    pub title : String,
    pub working_dir : String,
    pub editor: Editor
}

impl Project {
    pub fn from(id : String, title : String, working_dir : String, editor: Editor) -> Project {
        Project{
            id,
            title,
            working_dir,
            editor
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Projects{
    pub projects : Vec<Project>
}

