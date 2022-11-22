use super::{
    mesh_drawable::MeshDrawable,
    movable::Movable,
    player::Player,
    remove_when_below::RemoveWhenBelow,
    size::Size,
    weapon::{Weapon, WeaponType},
};
use crate::{
    camera::Camera,
    ecs::{
        entity::{Entity, EntitySystem},
        world::World,
    },
    models::WEAPON_PICKUP,
    sound_mixer::SoundMixer,
    sounds::PICKUP_1,
};
use game_derive::Component;
use n64_math::{random_u32, vec2, Aabb2, Quat, Vec2};
use strum::{EnumCount, IntoEnumIterator};

#[derive(Component)]
pub struct Pickup;

pub fn spawn_pickup(entities: &mut EntitySystem, start_pos: Vec2) -> Entity {
    entities
        .spawn()
        .add(Movable {
            pos: start_pos,
            speed: Vec2::new(0.0, 0.0),
        })
        .add(Size {
            size: WEAPON_PICKUP.size,
        })
        .add(MeshDrawable {
            model: WEAPON_PICKUP.as_model_data(),
            rot: Quat::IDENTITY,
        })
        .add(Pickup)
        .add(RemoveWhenBelow)
        .entity()
}

pub fn update(world: &mut World, sound_mixer: &mut SoundMixer, camera: &Camera) {
    let (pickup, movable, player, size, weapon) =
        world
            .components
            .get::<(Pickup, Movable, Player, Size, Weapon)>();

    let camera_bb = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

    for entity in pickup.entities() {
        if let (Some(m), Some(s)) = (movable.lookup(*entity), size.lookup(*entity)) {
            let mut delete = false;
            let pickup_bb = Aabb2::from_center_size(m.pos, s.size);

            if !pickup_bb.collides(&camera_bb) {
                delete = true;
            }

            for player_entity in player.entities() {
                if let (Some(player_movable), Some(player_size), Some(player_weapon)) = (
                    movable.lookup(*player_entity),
                    size.lookup(*player_entity),
                    weapon.lookup_mut(*player_entity),
                ) {
                    let player_bb = Aabb2::from_center_size(player_movable.pos, player_size.size);

                    if pickup_bb.collides(&player_bb) {
                        sound_mixer.play_sound(PICKUP_1.as_sound_data());
                        let weapon_index = random_u32() % WeaponType::COUNT as u32;
                        player_weapon.last_shoot_time = i64::MIN / 2;
                        player_weapon.weapon_type =
                            WeaponType::iter().nth(weapon_index as usize).unwrap();
                        delete = true;
                    }
                }
            }

            if delete {
                world.entities.despawn(*entity);
            }
        }
    }
}
