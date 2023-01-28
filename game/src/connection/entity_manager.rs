use super::receive_system::ServerReport;
use bevy::prelude::*;
#[derive(Resource)]
pub struct EntityManager {
    pub state: ServerReport,
}
