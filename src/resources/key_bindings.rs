use bevy::prelude::*;

#[derive(PartialEq)]
pub enum PlayerKey {
    Up,
    Down,
    Left,
    Right,
    RotateCw,
    RotateCcw,
    Fire,
}

impl PlayerKeyBinding {
    pub fn pressed(&self, key: Res<Input<KeyCode>>) -> Vec<PlayerKey> {
        let mut result = Vec::new();
        if self.up.iter().any(|k| key.pressed(*k)) {
            result.push(PlayerKey::Up);
        }
        if self.down.iter().any(|k| key.pressed(*k)) {
            result.push(PlayerKey::Down);
        }
        if self.left.iter().any(|k| key.pressed(*k)) {
            result.push(PlayerKey::Left);
        }
        if self.right.iter().any(|k| key.pressed(*k)) {
            result.push(PlayerKey::Right);
        }
        if self.rotate_cw.iter().any(|k| key.pressed(*k)) {
            result.push(PlayerKey::RotateCw);
        }
        if self.rotate_ccw.iter().any(|k| key.pressed(*k)) {
            result.push(PlayerKey::RotateCcw);
        }
        if self.fire.iter().any(|k| key.pressed(*k)) {
            result.push(PlayerKey::Fire);
        }
        result
    }
}

impl Default for PlayerKeyBinding {
    fn default() -> Self {
        Self {
            up: vec![KeyCode::W, KeyCode::Up],
            down: vec![KeyCode::S, KeyCode::Down],
            left: vec![KeyCode::A, KeyCode::Left],
            right: vec![KeyCode::D, KeyCode::Right],
            rotate_cw: vec![KeyCode::E],
            rotate_ccw: vec![KeyCode::Q],
            fire: vec![KeyCode::Space],
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct PauseMenuKeyBindings(pub Vec<KeyCode>);

impl Default for PauseMenuKeyBindings {
    fn default() -> Self {
        Self(vec![KeyCode::Escape, KeyCode::P])
    }
}

#[derive(Resource)]
pub struct PlayerKeyBinding {
    pub up: Vec<KeyCode>,
    pub down: Vec<KeyCode>,
    pub left: Vec<KeyCode>,
    pub right: Vec<KeyCode>,
    pub rotate_cw: Vec<KeyCode>,
    pub rotate_ccw: Vec<KeyCode>,
    pub fire: Vec<KeyCode>,
}
