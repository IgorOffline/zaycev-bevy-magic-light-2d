use bevy::asset::uuid_handle;
use bevy::prelude::*;

use crate::gi::compositing::PostProcessingMaterial;

pub const GI_SCREEN_PROBE_SIZE: i32 = 8;

pub const POST_PROCESSING_RECT: Handle<Mesh> = uuid_handle!("0eb8700e-e193-4c84-be83-803a6265b1a3");
pub const POST_PROCESSING_MATERIAL: Handle<PostProcessingMaterial> =
    uuid_handle!("a92bf31c-2d8d-4e69-b0fb-1aaeac31158a");
