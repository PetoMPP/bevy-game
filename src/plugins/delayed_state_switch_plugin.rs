use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct StateSetCommand {
    pub target: AppState,
    pub delay: Timer,
}

#[derive(Component)]
pub struct StatePushCommand {
    pub target: AppState,
    pub delay: Timer,
}

#[derive(Component)]
pub struct StatePopCommand {
    pub delay: Timer,
}

pub struct DelayedStateSwitchPlugin;

impl Plugin for DelayedStateSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(state_management_system);
    }
}

fn state_management_system(
    mut commands: Commands,
    time: Res<Time>,
    mut app_state: ResMut<State<AppState>>,
    mut pop_query: Query<(Entity, &mut StatePopCommand)>,
    mut set_query: Query<(Entity, &mut StateSetCommand)>,
    mut push_query: Query<(Entity, &mut StatePushCommand)>,
) {
    for (entity, mut pop_command) in pop_query.iter_mut() {
        pop_command.delay.tick(time.delta());
        if pop_command.delay.just_finished() {
            app_state.pop().unwrap();
            commands.entity(entity).despawn();
            return;
        }
    }
    for (entity, mut set_command) in set_query.iter_mut() {
        set_command.delay.tick(time.delta());
        if set_command.delay.just_finished() {
            app_state.set(set_command.target).unwrap();
            commands.entity(entity).despawn();
            return;
        }
    }
    for (entity, mut push_command) in push_query.iter_mut() {
        push_command.delay.tick(time.delta());
        if push_command.delay.just_finished() {
            app_state.push(push_command.target).unwrap();
            commands.entity(entity).despawn();
            return;
        }
    }
}
