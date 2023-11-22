use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

pub trait Identifiable<T> {
    fn get_id(&self) -> T;
}

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

impl Identifiable<u32> for Image {
    fn get_id(&self) -> u32 {
        self.id
    }
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

impl Identifiable<u32> for Annotation {
    fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Category {
    pub id: u32,
    pub supercategory: String,
    pub name: String,
}

impl Identifiable<u32> for Category {
    fn get_id(&self) -> u32 {
        self.id
    }
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

fn assert_id_uniqueness<'a, T, I>(c: T) -> Result<(), std::io::Error>
where
    T: Iterator<Item=&'a I> + Clone,
    I: Identifiable<u32> + 'a + Clone,
{
    let ids: HashSet<u32> = c.clone().map(|x| x.get_id()).collect();
    let all_ids: Vec<u32> = c.clone().map(|x| x.get_id()).collect();
    if ids.len() != all_ids.len() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Found non-unique ids."));
        // return Err(std::io::Error { "Found non unique ids." });
    }
    Ok(())
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

    fn validate_images(&self) -> Result<(), std::io::Error> {
        assert_id_uniqueness(self.images.iter())
    }

    fn validate_annotations(&self) -> Result<(), std::io::Error> {
        assert_id_uniqueness(self.annotations.iter())
    }

    fn validate_categories(&self) -> Result<(), std::io::Error> {
        assert_id_uniqueness(self.categories.iter())
    }

    pub fn validate(&self) -> Result<(), std::io::Error> {
        if let Err(e) = self.validate_images() {
            return Err(e);
        }
        if let Err(e) = self.validate_annotations() {
            return Err(e);
        };
        if let Err(e) = self.validate_categories() {
            return Err(e);
        }
        Ok(())
    }
	
	pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		let ds = serde_json::from_reader(reader)?;
		
	    Ok(ds)
	}

	pub fn dump_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        std::fs::write(path, serde_json::to_string_pretty(&self).unwrap())
    }
}
