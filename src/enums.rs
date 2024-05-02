/*
*
* This is where i put my enums, In the future this may be directory its own
* @Author : Lampros karachristos
*
*/

use std::fmt ;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Editor {
    IntelijUlt,
    IntelijCom,
    RustRover,
    VsCode,
    Nvim,
    Fleet,
    WebStorm,
    PlainText
}

impl fmt::Display for Editor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Editor::IntelijUlt => write!(f, "IntelijUlt"),
            Editor::IntelijCom => write!(f, "IntelijCom"),
            Editor::RustRover => write!(f, "RustRover"),
            Editor::VsCode => write!(f, "VsCode"),
            Editor::Nvim => write!(f, "Nvim"),
            Editor::Fleet => write!(f, "Fleet"),
            Editor::WebStorm => write!(f, "WebStorm"),
            Editor::PlainText => write!(f, "PlainText"),
        }
    }
}