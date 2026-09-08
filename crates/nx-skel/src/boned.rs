use std::{collections::BTreeMap, num::NonZeroU32};

#[derive(Debug, Clone)]
pub struct Bone {
    pub name: u32,
    pub parent: Option<NonZeroU32>,
    pub flip: Option<NonZeroU32>,
    pub quaternion: nx_common::Vec4,
    pub translation: nx_common::Vec4,
}

#[derive(Debug)]
pub struct Skeleton {
    bones: Vec<Bone>,
    by_name: BTreeMap<u32, Bone>,
}

impl Skeleton {
    pub fn new(bones: Vec<Bone>) -> Self {
        let by_name = bones
            .iter()
            .cloned()
            .map(|bone| (bone.name, bone))
            .collect();
        Self { bones, by_name }
    }

    pub fn get_index(&self, index: usize) -> Option<&Bone> {
        self.bones.get(index)
    }

    pub fn get_name(&self, name: u32) -> Option<&Bone> {
        self.by_name.get(&name)
    }
}
