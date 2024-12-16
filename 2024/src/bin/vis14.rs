use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(VisualizationPlugin {
			input: include_str!("../../input/2024/day14.txt")
				.trim()
				.to_string(),
		})
		.run();
}

#[derive(Component)]
struct Robot(aoc2024::day14::Robot);

struct VisualizationPlugin {
	input: String,
}
impl Plugin for VisualizationPlugin {
	fn build(&self, app: &mut App) {
		let input = self.input.clone();
		app.add_systems(
			Startup,
			move |commands: Commands,
			      meshes: ResMut<Assets<Mesh>>,
			      materials: ResMut<Assets<StandardMaterial>>| {
				add_robots(commands, meshes, materials, &input)
			},
		)
		.insert_resource(AmbientLight {
			color: Color::WHITE,
			brightness: 500.,
		});
	}
}
fn add_robots(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	input: &str,
) {
	let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
	let material = materials.add(Color::srgb_u8(255, 0, 0));
	commands.spawn_batch(aoc2024::day14::parse(input).into_iter().map(move |r| {
		let x = (r.p.x + 6285 * r.v.x).rem_euclid(101);
		let y = (r.p.y + 6285 * r.v.y).rem_euclid(103);
		(
			Transform::from_xyz(x as f32, y as f32, 0.5),
			Robot(r),
			Mesh3d(mesh.clone()),
			MeshMaterial3d(material.clone()),
		)
	}));
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(101.0 / 2.0, 103.0 / 2.0 + 70.0, 100.0)
			.looking_at(Vec3::new(101.0 / 2.0, 103.0 / 2.0 + 10.0, 0.0), Vec3::Z),
	));
	commands.spawn((
		Mesh3d(meshes.add(Rectangle::new(101.0, 103.0))),
		MeshMaterial3d(materials.add(Color::WHITE)),
		Transform::from_xyz(101.0 / 2.0, 103.0 / 2.0, 0.0),
	));
	commands.spawn((
		DirectionalLight {
			shadows_enabled: true,
			illuminance: 1000.0,
			..default()
		},
		Transform::from_xyz(40.0, 80.0, 40.0).looking_at(
			Vec3::new(101.0 / 2.0 + 20.0, 103.0 / 2.0 + 20.0, 150.0),
			Vec3::Z,
		),
	));
}
