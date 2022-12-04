use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Icon {
	pub path: String,
	pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub title: String,
    pub subtitle: String,
    pub arg: String,
    pub autocomplete: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Items {
    pub items: Vec<Item>,
}