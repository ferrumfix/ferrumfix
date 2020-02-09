use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Template {
    name: String,
    id: usize,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Templates {
    #[serde(rename = "template", default)]
    templates: Vec<Template>,
}
