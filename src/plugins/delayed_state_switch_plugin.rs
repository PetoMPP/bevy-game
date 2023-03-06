use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct StateSwitchCommand {
    pub target: AppState,
    pub delay: Timer,
}

pub struct DelayedStateSwitchPlugin;

impl Plugin for DelayedStateSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(state_switcher_system);
    }
}

fn state_switcher_system(
    mut commands: Commands,
    time: Res<Time>,
    mut app_state: ResMut<State<AppState>>,
    mut query: Query<(Entity, &mut StateSwitchCommand)>,
) {
    for (entity, mut switch_command) in query.iter_mut() {
        switch_command.delay.tick(time.delta());
        if switch_command.delay.just_finished() {
            app_state.set(switch_command.target).unwrap();
            commands.entity(entity).despawn();
        }
    }
}
