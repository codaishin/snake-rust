use bevy::{prelude::*, sprite::MaterialMesh2dBundle, sprite::Mesh2dHandle};
use rand::Rng;

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const STARTING_POSITION_INVISIBLE: Vec3 = Vec3::new(0.0, 0.0, -1.0);
const GRID_SCALE: f32 = 20.0;
const SEGMENT_SCALE: Vec3 = Vec3 {
	x: GRID_SCALE - 2.0,
	y: GRID_SCALE - 2.0,
	z: 0.0,
};
const りんご_SCALE: Vec3 = Vec3 {
	x: GRID_SCALE / 2.0,
	y: GRID_SCALE / 2.0,
	z: 0.0,
};
const PATH_COLOR: Color = Color::BEIGE;
const SNAKE_COLOR: Color = Color::GREEN;
const りんご_COLOR: Color = Color::RED;
const SPEED: f32 = 2.0;

struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_camera);
	}
}

fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

struct SnakePlugin;

impl Plugin for SnakePlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_snake)
			.add_system(update_path_and_りんご)
			.add_system(update_path_marker)
			.add_system(update_snake)
			.add_system(color_stomach)
			.add_system(update_direction);
	}
}

#[derive(Component)]
struct りんご;

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component, Deref, DerefMut)]
struct Path(Vec<Vec2>);

impl Path {
	fn goes_against(&self, vec: &Vec2) -> bool {
		(self[0] - self[1]).dot(*vec) < 0.0
	}

	fn shift_right(&mut self, vec: Vec2) {
		self.insert(0, vec);
		self.pop();
	}
}

trait ToVec3 {
	fn to_vec3(&self, z: f32) -> Vec3;
}

impl ToVec3 for Vec2 {
	fn to_vec3(&self, z: f32) -> Vec3 {
		(*self, z).into()
	}
}

#[derive(Component)]
struct PathScaler(f32);

#[derive(Component)]
struct PathIndex(usize);

#[derive(Component, PartialEq)]
struct SegmentIndex(usize);

#[derive(Component)]
struct Stomach(Vec<SegmentIndex>);

fn mesh2d(
	mesh: &Mesh2dHandle,
	material: &Handle<ColorMaterial>,
	position: Vec3,
	scale: Vec3,
) -> MaterialMesh2dBundle<ColorMaterial> {
	MaterialMesh2dBundle {
		mesh: mesh.clone(),
		material: material.clone(),
		transform: Transform::from_translation(position).with_scale(scale),
		..default()
	}
}

fn setup_snake(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let path = vec![
		Vec2 {
			x: GRID_SCALE,
			y: 0.0,
		},
		Vec2::ZERO,
	];
	let stomach: Vec<SegmentIndex> = vec![];
	let direction = Vec2 { x: 1.0, y: 0.0 };
	let path_mesh: Mesh2dHandle = meshes.add(shape::Quad::default().into()).into();
	let snake_mesh: Mesh2dHandle = meshes.add(shape::Circle::default().into()).into();
	let りんご_mesh: Mesh2dHandle = meshes.add(shape::Circle::default().into()).into();
	let path_color = materials.add(ColorMaterial::from(PATH_COLOR));
	let snake_color = materials.add(ColorMaterial::from(SNAKE_COLOR));
	let りんご_color = materials.add(ColorMaterial::from(りんご_COLOR));

	commands.spawn((
		Path(path),
		PathScaler(0.0),
		Direction(direction),
		Stomach(stomach),
	));
	commands.spawn((
		mesh2d(&path_mesh, &path_color, STARTING_POSITION, SEGMENT_SCALE),
		PathIndex(0),
	));
	commands.spawn((
		mesh2d(&path_mesh, &path_color, STARTING_POSITION, SEGMENT_SCALE),
		PathIndex(1),
	));
	commands.spawn((
		mesh2d(&snake_mesh, &snake_color, STARTING_POSITION, SEGMENT_SCALE),
		SegmentIndex(0),
	));
	commands.spawn((
		mesh2d(
			&りんご_mesh,
			&りんご_color,
			Vec3 {
				x: 10.0 * GRID_SCALE,
				y: 10.0 * GRID_SCALE,
				z: 0.0,
			},
			りんご_SCALE,
		),
		りんご,
	));
}

fn update_path_marker(
	mut query_markers: Query<(&mut Transform, &PathIndex)>,
	query_path: Query<&Path>,
) {
	let path = query_path.single();
	for (mut transform, index) in query_markers.iter_mut() {
		let position: Vec3 = path[index.0].to_vec3(0.0);
		transform.translation = position;
	}
}

fn update_snake(
	mut query_segments: Query<(&mut Transform, &SegmentIndex)>,
	query_path: Query<(&Path, &PathScaler)>,
) {
	let (path, scaler) = query_path.single();
	for (mut transform, index) in query_segments.iter_mut() {
		let start: Vec3 = path[index.0 + 1].to_vec3(1.0);
		let target: Vec3 = path[index.0].to_vec3(1.0);
		transform.translation = start.lerp(target, scaler.0);
	}
}

fn color_stomach(
	query_stomach: Query<&Stomach>,
	query_snake: Query<(&Handle<ColorMaterial>, &SegmentIndex)>,
	mut assets: ResMut<Assets<ColorMaterial>>,
) {
	let stomach = query_stomach.single();
	for (color_handle, index) in query_snake.iter() {
		let mut material = assets
			.get_mut(&color_handle)
			.expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
		material.color = if stomach.0.contains(index) {
			Color::PINK
		} else {
			SNAKE_COLOR
		}
	}
}

fn update_path_and_りんご(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut query_path: Query<(&mut PathScaler, &mut Path, &Direction, &mut Stomach)>,
	mut query_りんご: Query<&mut Transform, With<りんご>>,
	time: Res<Time>,
) {
	let (mut scaler, mut path, direction, mut stomach) = query_path.single_mut();
	let mut _りんご = query_りんご.single_mut();
	let delta = time.delta().as_secs_f32() * SPEED;
	let new_scale = scaler.0 + delta;
	scaler.0 = new_scale % 1.0;

	if new_scale < 1.0 {
		return;
	}

	let fst = path[0];
	let new_head = fst + direction.0 * GRID_SCALE;
	let current_last_index = path.0.len() - 2;

	if stomach.0.len() > 0 && stomach.0[0].0 == current_last_index {
		path.0.insert(0, new_head);

		let path_mesh: Mesh2dHandle = meshes.add(shape::Quad::default().into()).into();
		let snake_mesh: Mesh2dHandle = meshes.add(shape::Circle::default().into()).into();
		let path_color = materials.add(ColorMaterial::from(PATH_COLOR));
		let snake_color = materials.add(ColorMaterial::from(SNAKE_COLOR));

		commands.spawn((
			mesh2d(
				&path_mesh,
				&path_color,
				STARTING_POSITION_INVISIBLE,
				SEGMENT_SCALE,
			),
			PathIndex(current_last_index + 2),
		));
		commands.spawn((
			mesh2d(
				&snake_mesh,
				&snake_color,
				STARTING_POSITION_INVISIBLE,
				SEGMENT_SCALE,
			),
			SegmentIndex(current_last_index + 1),
		));
		stomach.0.remove(0);
	} else {
		path.shift_right(new_head);
	}

	for elem in &mut stomach.0 {
		elem.0 += 1;
	}

	if _りんご.translation != new_head.to_vec3(0.0) {
		return;
	}

	stomach.0.push(SegmentIndex(0));

	let mut rng = rand::thread_rng();

	_りんご.translation = Vec3 {
		x: GRID_SCALE * (rng.gen_range(-20..=20) as f32),
		y: GRID_SCALE * (rng.gen_range(-10..=10) as f32),
		z: 0.0,
	}
}

fn direction_from_input(keyboard_input: Res<Input<KeyCode>>) -> Option<Vec2> {
	if keyboard_input.pressed(KeyCode::Left) {
		return Some(Vec2 { x: -1.0, y: 0.0 });
	}
	if keyboard_input.pressed(KeyCode::Right) {
		return Some(Vec2 { x: 1.0, y: 0.0 });
	}
	if keyboard_input.pressed(KeyCode::Up) {
		return Some(Vec2 { x: 0.0, y: 1.0 });
	}
	if keyboard_input.pressed(KeyCode::Down) {
		return Some(Vec2 { x: 0.0, y: -1.0 });
	}
	return None;
}

fn update_direction(
	keyboard_input: Res<Input<KeyCode>>,
	mut query_path: Query<(&Path, &mut Direction)>,
) {
	let (path, mut direction) = query_path.single_mut();
	let new_direction = match direction_from_input(keyboard_input) {
		Some(v) => v,
		None => return,
	};

	if path.goes_against(&new_direction) {
		return;
	}
	direction.0 = new_direction;
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(CameraPlugin)
		.add_plugin(SnakePlugin)
		.run();
}
