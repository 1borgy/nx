use nx_bone::{Quaternion, Translation};
use std::{collections::BTreeMap, num::NonZeroU32};

#[derive(Debug, Clone)]
pub struct Bone {
    pub name: u32,
    pub parent: Option<NonZeroU32>,
    pub flip: Option<NonZeroU32>,
    pub quaternion: Quaternion,
    pub translation: Translation,
}

impl Bone {
    pub fn empty() -> Self {
        Self {
            name: 0,
            parent: None,
            flip: None,
            quaternion: Quaternion::identity(),
            translation: Translation::identity(),
        }
    }
}

#[derive(Debug)]
pub struct Skeleton {
    pub bones: Vec<Bone>,
    // by_name: BTreeMap<u32, Bone>,
}

impl Skeleton {
    pub fn new(bones: Vec<Bone>) -> Self {
        // let by_name = bones
        //     .iter()
        //     .cloned()
        //     .map(|bone| (bone.name, bone))
        //     .collect();
        Self {
            bones,
            //by_name
        }
    }

    pub fn bone(&self, index: usize) -> Bone {
        self.bones.get(index).cloned().unwrap_or(Bone::empty())
    }

    pub fn get_index(&self, index: usize) -> Option<&Bone> {
        self.bones.get(index)
    }

    // pub fn get_name(&self, name: u32) -> Option<&Bone> {
    //     self.by_name.get(&name)
    // }
}
