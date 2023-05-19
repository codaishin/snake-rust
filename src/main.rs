use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const TIME_STEP: f32 = 1.0 / 60.0;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
// These values are exact
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Resource)]
struct Scoreboard {
	score: usize,
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
			.insert_resource(Scoreboard { score: 0 })
			.insert_resource(ClearColor(BACKGROUND_COLOR))
			.add_startup_system(add_people)
			.add_startup_system(add_text_window)
			.add_system(update_scoreboard)
			.add_system(greet_people)
			.add_system(bevy::window::close_on_esc);
	}
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
	commands.spawn((Person, Name("Elaina Proctor".into())));
	commands.spawn((Person, Name("Renzo Hume".into())));
	commands.spawn((Person, Name("Zayna Nieves".into())));
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
	let mut text = query.single_mut();
	text.sections[1].value = scoreboard.score.to_string();
	// println!("{}", text.sections[1].value);
}

fn add_text_window(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(Camera2dBundle::default());
	commands.spawn(
		TextBundle::from_sections([
			TextSection::new(
				"Score: ",
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: SCOREBOARD_FONT_SIZE,
					color: TEXT_COLOR,
				},
			),
			TextSection::from_style(TextStyle {
				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
				font_size: SCOREBOARD_FONT_SIZE,
				color: Color::BLUE,
			}),
			TextSection::from_style(TextStyle {
				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
				font_size: SCOREBOARD_FONT_SIZE,
				color: Color::BLUE,
			}),
			TextSection::from_style(TextStyle {
				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
				font_size: SCOREBOARD_FONT_SIZE,
				color: Color::BLUE,
			}),
			TextSection::from_style(TextStyle {
				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
				font_size: SCOREBOARD_FONT_SIZE,
				color: Color::BLUE,
			}),
		])
		.with_style(Style {
			position_type: PositionType::Absolute,
			position: UiRect {
				top: SCOREBOARD_TEXT_PADDING,
				left: SCOREBOARD_TEXT_PADDING,
				..default()
			},
			..default()
		}),
	);
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
	mut scoreboard: ResMut<Scoreboard>,
	time: Res<Time>,
	mut timer: ResMut<GreetTimer>,
	query: Query<&Name, With<Person>>,
	mut text_query: Query<&mut Text>,
) {
	if !timer.0.tick(time.delta()).just_finished() {
		return;
	}
	let mut i = 2;
	let mut text = text_query.single_mut();
	for name in &query {
		text.sections[i].value = format!("hello {}!", name.0);
		i += 1;
	}
	scoreboard.score += 1;
}

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
		app.add_startup_system(setup_player)
			.add_system(update_velocity.before(update_player_position))
			.add_system(update_player_position);
	}
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Direction(Vec2);

fn setup_player(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn((
		MaterialMesh2dBundle {
			mesh: meshes.add(shape::Circle::default().into()).into(),
			material: materials.add(ColorMaterial::from(BALL_COLOR)),
			transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
			..default()
		},
		Player,
		Direction(Vec2 { x: 0.0, y: 0.0 }),
	));
}

fn update_player_position(
	mut query: Query<(&mut Transform, &Direction), With<Player>>,
	time: Res<Time>,
) {
	let (mut transform, velocity) = query.single_mut();
	let translation_velocity: Vec3 = (velocity.0, 0.0).into();
	transform.translation += translation_velocity * time.delta().as_secs_f32() * 100.0;
}

fn update_velocity(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<&mut Direction, With<Player>>,
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
