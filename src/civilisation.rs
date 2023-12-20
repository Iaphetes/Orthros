use std::collections::HashMap;

use bevy::prelude::*;

use crate::{resources::ResourceType, spawner::Civilisation};

pub struct EcoBoni {
    resource_boni: HashMap<ResourceType, f32>,
}
pub struct CivilisationBoni {
    eco_boni: EcoBoni,
}
#[derive(Resource)]
pub struct CivilisationBoniMap {
    map: HashMap<Civilisation, CivilisationBoni>,
}

fn setup_civilisations(mut commands: Commands) {
    let mut civ_boni_map = CivilisationBoniMap {
        map: HashMap::new(),
    };
    civ_boni_map.map.insert(
        Civilisation::Greek,
        CivilisationBoni {
            eco_boni: EcoBoni {
                resource_boni: HashMap::from_iter([(ResourceType::Plotanium, 1.2)]),
            },
        },
    );
    commands.insert_resource(civ_boni_map);
}
pub struct CivilisationPlugin;

impl Plugin for CivilisationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_civilisations);
    }
}
