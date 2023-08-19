use bevy_ggrs::ggrs;
use bevy_matchbox::prelude::*;

pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    // A single byte should fit a bunch of inputs
    type Input = u8;
    type State = u8;
    
    type Address = PeerId;
}
