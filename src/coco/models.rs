use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub struct Image {
    pub id: u32,
    pub file_name: String,
    pub date_captured: String,
    pub width: u32,
    pub height: u32,

    #[serde(default)]
    pub coco_url: String,

    #[serde(default)]
    pub license: u32,

    #[serde(default)]
    pub flickr_url: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Annotation {
    pub id: u32,
    pub image_id: u32,
    pub category_id: u32,
    pub bbox: (f32, f32, f32, f32),

    #[serde(default)]
    pub area: f32,

    #[serde(default)]
    pub iscrowd: u8,

    #[serde(default = "Vec::<Vec<f32>>::new")]
    pub segmentation: Vec<Vec<f32>>,

    #[serde(default = "HashMap::<String, String>::new")]
    pub attributes: HashMap<String, String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Category {
    pub id: u32,
    pub supercategory: String,
    pub name: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Info {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub version: String,

    #[serde(default)]
    pub date: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub author: String,
}

impl Info {
    pub fn new() -> Self {
        Info {
            name: String::new(),
            version: "0.1.0".to_string(),
            date: Utc::now().to_string(),
            description: String::new(),
            author: String::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct License {
    pub id: u32,
    pub name: String,

    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Dataset {
    pub images: Vec<Image>,
    pub annotations: Vec<Annotation>,
    pub categories: Vec<Category>,

    #[serde(default = "Info::new")]
    pub info: Info,

    #[serde(default = "Vec::<License>::new")]
    pub licenses: Vec<License>,
}

impl Dataset {
    pub fn new(
        images: Vec<Image>,
        annotations: Vec<Annotation>,
        categories: Vec<Category>,
        info: Info,
        licenses: Vec<License>,
    ) -> Self {
        Dataset {
            images,
            annotations,
            categories,
            info,
            licenses,
        }
    }
}
