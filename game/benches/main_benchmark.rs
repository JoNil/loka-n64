use criterion::{criterion_group, criterion_main, Criterion};
use game::{
    camera::Camera,
    components::{
        diver_ai, enemy, health, keep_on_screen, missile, movable,
        pickup::{self, spawn_pickup},
        player::{self, spawn_player},
        print_position, projectile, remove_when_below, spawner, trap, waypoint_ai,
    },
    ecs::world::World,
    map::Map,
    maps::MAP_1,
    sound_mixer::SoundMixer,
};
use n64::{Controllers, VideoMode};
use n64_math::vec2;

const VIDEO_MODE: VideoMode = VideoMode::Pal {
    width: 320,
    height: 240,
};

fn criterion_benchmark(c: &mut Criterion) {
    let mut world = World::new();
    let map = Map::load(MAP_1);

    let start_pos = vec2(
        map.get_start_pos().x / VIDEO_MODE.width() as f32,
        map.get_start_pos().y / VIDEO_MODE.height() as f32 - 1.0,
    );

    let controllers = Controllers::new();
    let mut sound_mixer = SoundMixer::new();
    let camera = Camera::new(start_pos);
    let _test_pickup = spawn_pickup(&mut world.entities, start_pos + vec2(0.5, 0.2));
    let _player = spawn_player(&mut world.entities, start_pos);
    map.spawn_enemies(&mut world, &VIDEO_MODE);

    let dt = 1.0 / 60.0;

    c.bench_function("game", |b| {
        b.iter(|| {
            health::clear_was_damaged(&mut world);
            enemy::update(&mut world, &mut sound_mixer);
            player::update(&mut world, &controllers, &mut sound_mixer, &camera);
            diver_ai::update(&mut world);
            waypoint_ai::update(&mut world, dt);
            missile::update(&mut world, dt);
            movable::simulate(&mut world, dt);
            projectile::update(&mut world, &mut sound_mixer, &camera, dt);
            trap::update(&mut world);
            pickup::update(&mut world, &mut sound_mixer, &camera);
            spawner::update(&mut world, &camera);
            keep_on_screen::update(&mut world, &camera);
            remove_when_below::update(&mut world, &camera);
            print_position::print(&mut world);
            world.housekeep();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
