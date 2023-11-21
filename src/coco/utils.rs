use crate::coco::models;
use std::collections::HashSet;
use std::path::{PathBuf, Path};

pub fn merge_drain(mut datasets: Vec<models::Dataset>) -> models::Dataset {
    let mut images: Vec<models::Image> = Vec::new();
    let mut annotations: Vec<models::Annotation> = Vec::new();
    let mut categories: Vec<models::Category> = Vec::new();
    let mut licenses: Vec<models::License> = Vec::new();

    for ds in datasets.iter_mut() {
        images.extend(ds.images.drain(..));
        annotations.extend(ds.annotations.drain(..));
        categories.extend(ds.categories.drain(..));
        licenses.extend(ds.licenses.drain(..));
    }

    models::Dataset::new(
        images,
        annotations,
        categories,
        models::Info::new(),
        licenses,
    )
}

pub fn merge_clone(datasets: Vec<models::Dataset>) -> models::Dataset {
    let mut images: Vec<models::Image> = Vec::new();
    let mut annotations: Vec<models::Annotation> = Vec::new();
    let mut categories: Vec<models::Category> = Vec::new();
    let mut licenses: Vec<models::License> = Vec::new();

    for ds in datasets.iter() {
        images.extend(ds.images.iter().cloned());
        annotations.extend(ds.annotations.iter().cloned());
        categories.extend(ds.categories.iter().cloned());
        licenses.extend(ds.licenses.iter().cloned());
    }

    models::Dataset::new(
        images,
        annotations,
        categories,
        models::Info::new(),
        licenses,
    )
}

impl models::Dataset {
    pub fn rebase<P: AsRef<Path>>(&mut self, base_path: P) -> () {
        for i in self.images.iter_mut() {
            i.file_name = base_path.as_ref()
                .join(i.file_name.clone())
                .iter().collect::<PathBuf>()
                .as_os_str()
                .to_str()
                .expect("")
                .to_owned();
        }
    }

    pub fn image_split(&self, split: f32) -> (models::Dataset, models::Dataset) {
        let n: usize = (split * self.images.len() as f32).floor() as usize;

        let image_ids: HashSet<u32> = self.images.iter().map(|image| image.id).collect();

        let image_ids1: HashSet<u32> = image_ids.iter().take(n).copied().collect();
        let image_ids2: HashSet<u32> = image_ids.difference(&image_ids).copied().collect();

        let images: Vec<models::Image> = self
            .images
            .iter()
            .filter(|i| image_ids1.contains(&i.id))
            .cloned()
            .collect();
        let annotations: Vec<models::Annotation> = self
            .annotations
            .iter()
            .filter(|a| image_ids1.contains(&a.image_id))
            .cloned()
            .collect();
        let categories_ids: HashSet<u32> = annotations.iter().map(|a| a.category_id).collect();
        let categories: Vec<models::Category> = self
            .categories
            .iter()
            .filter(|c| categories_ids.contains(&c.id))
            .cloned()
            .collect();

        let first_split = models::Dataset {
            images,
            annotations,
            categories,
            info: models::Info::new(),
            licenses: vec![],
        };

        let images: Vec<models::Image> = self
            .images
            .iter()
            .filter(|i| image_ids2.contains(&i.id))
            .cloned()
            .collect();
        let annotations: Vec<models::Annotation> = self
            .annotations
            .iter()
            .filter(|a| image_ids2.contains(&a.image_id))
            .cloned()
            .collect();
        let categories_ids: HashSet<u32> = annotations.iter().map(|a| a.category_id).collect();
        let categories: Vec<models::Category> = self
            .categories
            .iter()
            .filter(|c| categories_ids.contains(&c.id))
            .cloned()
            .collect();

        let second_split = models::Dataset {
            images,
            annotations,
            categories,
            info: models::Info::new(),
            licenses: vec![],
        };

        (first_split, second_split)
    }

    pub fn annotation_split(&self) -> (models::Dataset, models::Dataset) {
        (self.clone(), self.clone())
    }
}
