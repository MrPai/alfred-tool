use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Icon {
	pub path: String,
	pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub uid: Option<String>,
    pub valid: Option<bool>,
    pub r#type: Option<String>,
    pub title: String,
    pub subtitle: String,
    pub arg: String,
    pub autocomplete: String,
    pub icon: Option<Icon>,
}

impl Item {
    pub fn new(title: String,subtitle: String,arg: String,autocomplete: String)->Self{
        Item {
            uid: None,
            valid: None,
            r#type: None,
            title,
            subtitle,
            arg,
            autocomplete,
            icon: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Items {
    pub items: Vec<Item>,
}