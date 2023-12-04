use block_mesh::{
    ndshape::{ConstShape, ConstShape3u32},
    MergeVoxel, Voxel, VoxelVisibility,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoolVoxel(bool);

pub const EMPTY: BoolVoxel = BoolVoxel(false);
pub const FULL: BoolVoxel = BoolVoxel(true);

impl Voxel for BoolVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == EMPTY {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for BoolVoxel {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}

pub type ChunkShape = ConstShape3u32<66, 66, 66>;
pub const CHUNK_SHAPE_SIZE: usize = ChunkShape::SIZE as usize;

pub fn gen_voxels() -> [BoolVoxel; CHUNK_SHAPE_SIZE] {
    let mut voxels = [EMPTY; CHUNK_SHAPE_SIZE];

    for i in 0..CHUNK_SHAPE_SIZE {
        let [x, y, z] = ChunkShape::delinearize(i as u32);
        let [x, y, z] = [x as i32 - 32, y as i32 - 32, z as i32 - 32];
        if x * x + y * y + z * z < 1024 {
            voxels[i] = FULL;
        }
    }

    voxels
}
