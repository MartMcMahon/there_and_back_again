use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Default, Resource)]
pub struct DebugValue(pub Vec3);

pub struct DebugText;
impl Plugin for DebugText {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup)
            .add_system(text_update_system)
            .add_system(text_color_system);

        // app.insert_resource(Value(self.value));

        // if let TrackedValue(Some(value)) = self.tracked_value {
        //     app.insert_resource(self.tracked_value)
        //     .add_system(tracking_system);
        // }
    }
}

#[derive(Component)]
struct FpsText;
#[derive(Component)]
struct ColorText;

const REGULAR_FONT_PATH: &str = "fonts/Roboto-Medium.ttf"; // FiraSans-Bold.ttf
                                                           // const BOLD_FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "hello\nbevy!",
            TextStyle {
                font: asset_server.load(REGULAR_FONT_PATH),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load(REGULAR_FONT_PATH),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(REGULAR_FONT_PATH),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ]),
        FpsText,
    ));
}

fn text_color_system(
    time: Res<Time>,
    value: Res<DebugValue>,
    mut query: Query<&mut Text, With<ColorText>>,
) {
    let val = value.0;
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();
        text.sections[0].value = format!("g: {:.3}, {:.3}", val.z, val.y);

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
