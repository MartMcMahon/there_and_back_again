fn setup_hexagon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Hexagon
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(10., 1., 1.))
                .with_rotation(Quat::from_xyzw(0.0, 1.0, 0.0, 0.0)),
            ..default()
        })
        .insert(Player)
        .insert(Body {
            mass: 0.1,
            vel: Vec3::ZERO,
        });
}
fn setup_triangle_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // Positions of the vertices
    // See https://bevy-cheatbook.github.io/features/coords.html
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.5, 0., -0.5],
            [-0.5, 0., 0.5],
            [0.5, 0., 0.5],
            [0.5, 0., -0.5],
            [0., 1., 0.],
        ],
    );

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 5]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 5]);

    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    mesh.set_indices(Some(mesh::Indices::U32(vec![
        0, 1, 4, 1, 2, 4, 0, 3, 4, 2, 3, 4,
    ])));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(10.0, 1.0, -1.0),
            // .with_rotation(Quat::from_rotation_y(-0.25 * PI)),
            ..default()
        })
        .insert(Player)
        .insert(Body {
            mass: 0.1,
            vel: Vec3::ZERO,
        });
}

// commands
//     .spawn(SceneBundle {
//         scene: asset_server.load("models/pyramid.glb#Scene0"),
//         transform: Transform {
//             translation: Vec3::ZERO,
//             scale: Vec3 {
//                 x: 0.05,
//                 y: 0.05,
//                 z: 0.05,
//             },
//             ..default()
//         },
//         ..default()
//     })
//     .insert(Player)
//     .insert(Body {
//         mass: 0.1,
//         vel: Vec3::ZERO,
//     });
