use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_camera);
	}
}

fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_snake)
			.add_system(update_path_marker)
			.add_system(update_snake)
			.add_system(update_path);
	}
}

#[derive(Component)]
struct Snake;

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Path(Vec<Vec2>);

#[derive(Component)]
struct PathScaler(f32);

#[derive(Component)]
struct PathIndex(usize);

#[derive(Component)]
struct SegmentIndex(usize);

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const GRID_SCALE: f32 = 20.0;
const PATH_COLOR: Color = Color::BEIGE;
const SNAKE_COLOR: Color = Color::GREEN;
const SPEED: f32 = 1.0;

fn setup_snake(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn((
		Path(vec![
			Vec2 {
				x: GRID_SCALE,
				y: 0.0,
			},
			Vec2 { x: 0.0, y: 0.0 },
		]),
		PathScaler(0.0),
	));
	commands.spawn((
		MaterialMesh2dBundle {
			mesh: meshes.add(shape::Quad::default().into()).into(),
			material: materials.add(ColorMaterial::from(PATH_COLOR)),
			transform: Transform::from_translation(STARTING_POSITION).with_scale(Vec3 {
				x: GRID_SCALE - 2.0,
				y: GRID_SCALE - 2.0,
				z: 0.0,
			}),
			..default()
		},
		PathIndex(0),
	));
	commands.spawn((
		MaterialMesh2dBundle {
			mesh: meshes.add(shape::Quad::default().into()).into(),
			material: materials.add(ColorMaterial::from(PATH_COLOR)),
			transform: Transform::from_translation(STARTING_POSITION).with_scale(Vec3 {
				x: GRID_SCALE - 2.0,
				y: GRID_SCALE - 2.0,
				z: 0.0,
			}),
			..default()
		},
		PathIndex(1),
	));
	commands.spawn((
		MaterialMesh2dBundle {
			mesh: meshes.add(shape::Circle::default().into()).into(),
			material: materials.add(ColorMaterial::from(SNAKE_COLOR)),
			transform: Transform::from_translation(STARTING_POSITION).with_scale(Vec3 {
				x: GRID_SCALE - 2.0,
				y: GRID_SCALE - 2.0,
				z: 0.0,
			}),
			..default()
		},
		SegmentIndex(0),
	));
}

fn update_path_marker(
	mut query_markers: Query<(&mut Transform, &PathIndex)>,
	query_path: Query<&Path>,
) {
	let path = query_path.single();
	for (mut transform, index) in query_markers.iter_mut() {
		let position: Vec3 = (path.0[index.0], 0.0).into();
		transform.translation = position;
	}
}

fn update_snake(
	mut query_segments: Query<(&mut Transform, &SegmentIndex)>,
	query_path: Query<(&Path, &PathScaler)>,
) {
	let (path, scaler) = query_path.single();
	for (mut transform, index) in query_segments.iter_mut() {
		let start: Vec3 = (path.0[index.0 + 1], 1.0).into();
		let target: Vec3 = (path.0[index.0], 1.0).into();
		transform.translation = start.lerp(target, scaler.0);
	}
}

fn update_path(mut query_scaler: Query<&mut PathScaler>, time: Res<Time>) {
	let mut scaler = query_scaler.single_mut();
	let delta = time.delta().as_secs_f32() * SPEED;
	scaler.0 = (scaler.0 + delta) % 1.0;
}

fn update_velocity(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<&mut Direction, With<Snake>>,
) {
	let mut direction = query.single_mut();
	if keyboard_input.pressed(KeyCode::Left) {
		direction.0 = Vec2 { x: -1.0, y: 0.0 };
	}
	if keyboard_input.pressed(KeyCode::Right) {
		direction.0 = Vec2 { x: 1.0, y: 0.0 };
	}
	if keyboard_input.pressed(KeyCode::Up) {
		direction.0 = Vec2 { x: 0.0, y: 1.0 };
	}
	if keyboard_input.pressed(KeyCode::Down) {
		direction.0 = Vec2 { x: 0.0, y: -1.0 };
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(CameraPlugin)
		.add_plugin(PlayerPlugin)
		.run();
}
