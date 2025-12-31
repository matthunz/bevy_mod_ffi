use bevy_reflect::TypePath;
use bytemuck::Pod;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Table,
    SparseSet,
}

pub trait Component: Pod + TypePath + Sized {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}
