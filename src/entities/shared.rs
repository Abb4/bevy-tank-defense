use bevy::prelude::*;

#[derive(Component, Default)]
pub struct DisplayName(pub String);

#[derive(Component)]
pub struct Movable {
    pub speed: i32,
}

impl Default for Movable {
    fn default() -> Self {
        Self { speed: 5 }
    }
}

#[derive(Bundle)]
pub struct EntitySharedBundle {
    pub name: DisplayName,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl Default for EntitySharedBundle {
    fn default() -> Self {
        let default_sprite = Sprite {
            custom_size: Some(Vec2 { x: 64.0, y: 64.0 }),
            ..Default::default()
        };

        Self {
            name: DisplayName("Unnamed Entity".to_string()),
            sprite: SpriteBundle {
                sprite: default_sprite,
                ..Default::default()
            },
        }
    }
}
