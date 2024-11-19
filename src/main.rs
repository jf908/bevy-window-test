use bevy::{
    color::palettes::tailwind,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    window::{Monitor, PresentMode, WindowMode},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_ui, update_buttons, update_monitor_buttons))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 14.0,
                    ..default()
                },
                ..default()
            },
        })
        .run();
}

#[derive(Component)]
pub struct MonitorNode;

#[derive(Component)]
enum TextMode {
    Monitor,
    PresentMode,
    WindowMode,
    WindowPosition,
    PhysicalWindowSize,
    LogicalWindowSize,
    ScaleFactor,
}

#[derive(Component)]
enum ButtonType {
    SetWindowMode(WindowMode),
    SetResolution((u32, u32)),
    SetScaleFactor(f32),
    SetPosition(WindowPosition),
    SetPresentMode(PresentMode),
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let font = TextFont {
        font_size: 16.0,
        ..default()
    };

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            margin: UiRect::top(Val::Px(8.0)),
            padding: UiRect::all(Val::Px(16.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    create_button(
                        parent,
                        ButtonType::SetWindowMode(WindowMode::Windowed),
                        "Set Windowed",
                    );
                    create_button(
                        parent,
                        ButtonType::SetWindowMode(WindowMode::BorderlessFullscreen(
                            MonitorSelection::Current,
                        )),
                        "Set BorderlessFullscreen",
                    );
                    create_button(
                        parent,
                        ButtonType::SetWindowMode(WindowMode::SizedFullscreen(
                            MonitorSelection::Current,
                        )),
                        "Set SizedFullscreen",
                    );
                    create_button(
                        parent,
                        ButtonType::SetWindowMode(WindowMode::Fullscreen(
                            MonitorSelection::Current,
                        )),
                        "Set Fullscreen",
                    );
                });

            parent
                .spawn(Node {
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    create_button(
                        parent,
                        ButtonType::SetPresentMode(PresentMode::AutoVsync),
                        "Set AutoVsync",
                    );
                    create_button(
                        parent,
                        ButtonType::SetPresentMode(PresentMode::AutoNoVsync),
                        "Set AutoNoVsync",
                    );
                });

            parent
                .spawn(Node {
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    create_button(parent, ButtonType::SetResolution((800, 600)), "Set 800x600");
                    create_button(
                        parent,
                        ButtonType::SetResolution((1280, 720)),
                        "Set 1280x720",
                    );
                    create_button(
                        parent,
                        ButtonType::SetResolution((1920, 1080)),
                        "Set 1920x1080",
                    );
                });

            parent
                .spawn(Node {
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    create_button(parent, ButtonType::SetScaleFactor(1.0), "Set 1x scaling");
                    create_button(parent, ButtonType::SetScaleFactor(1.5), "Set 1.5x scaling");
                    create_button(parent, ButtonType::SetScaleFactor(2.0), "Set 2x scaling");
                });

            parent.spawn((
                MonitorNode,
                Node {
                    column_gap: Val::Px(8.0),
                    ..default()
                },
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_child((TextMode::WindowMode, Text::new(""), font.clone()))
                .with_child((TextMode::PresentMode, Text::new(""), font.clone()))
                .with_child((TextMode::WindowPosition, Text::new(""), font.clone()))
                .with_child((TextMode::PhysicalWindowSize, Text::new(""), font.clone()))
                .with_child((TextMode::LogicalWindowSize, Text::new(""), font.clone()))
                .with_child((TextMode::ScaleFactor, Text::new(""), font.clone()))
                .with_child((TextMode::Monitor, Text::new(""), font.clone()));
        });
}

#[derive(Default)]
struct MonitorCache(usize);

fn update_monitor_buttons(
    mut commands: Commands,
    monitor_node: Single<(Entity, Option<&Children>), With<MonitorNode>>,
    monitors: Query<&Monitor>,
    mut monitor_len: Local<MonitorCache>,
) {
    if monitor_len.0 == monitors.iter().len() {
        return;
    }

    monitor_len.0 = monitors.iter().len();

    if let Some(children) = monitor_node.1 {
        for child in children.iter() {
            commands.entity(*child).despawn_recursive();
        }
    }

    let mut parent = commands.entity(monitor_node.0);

    parent.with_children(|parent| {
        create_button(
            parent,
            ButtonType::SetPosition(WindowPosition::Centered(MonitorSelection::Primary)),
            &format!("Move to primary monitor"),
        );

        for (i, _) in monitors.iter().enumerate() {
            create_button(
                parent,
                ButtonType::SetPosition(WindowPosition::Centered(MonitorSelection::Index(i))),
                &format!("Move to monitor {}", i),
            );
        }
    });
}

fn update_ui(
    mut res_text: Query<(&mut Text, &TextMode)>,
    window: Single<&Window>,
    monitors: Query<&Monitor>,
) {
    for (mut text, mode) in &mut res_text {
        text.0 = match mode {
            TextMode::Monitor => {
                format!(
                    "Monitors: {}",
                    monitors
                        .iter()
                        .map(|m| monitor_to_string(m))
                        .collect::<Vec<_>>()
                        .join("")
                )
            }
            TextMode::PresentMode => {
                format!("Present mode: {:?}", window.present_mode)
            }
            TextMode::WindowMode => {
                format!("Window mode: {:?}", window.mode)
            }
            TextMode::WindowPosition => {
                format!("Window position: {:?}", window.position)
            }
            TextMode::PhysicalWindowSize => {
                format!(
                    "Physical Resolution: {}x{}",
                    window.physical_width(),
                    window.physical_height()
                )
            }
            TextMode::LogicalWindowSize => {
                format!("Logical resolution: {}x{}", window.width(), window.height())
            }
            TextMode::ScaleFactor => {
                format!("Scale factor: {}", window.scale_factor())
            }
        }
    }
}

fn monitor_to_string(monitor: &Monitor) -> String {
    format!(
        "\n\t{} {}x{}@{} {}x",
        monitor.name.as_deref().unwrap_or("No name"),
        monitor.physical_width,
        monitor.physical_height,
        (monitor.refresh_rate_millihertz.unwrap_or(0) as f64) / 1000.0,
        monitor.scale_factor
    )
}

fn update_buttons(
    mut button_query: Query<
        (&Interaction, &ButtonType, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut window: Single<&mut Window>,
) {
    for (interaction, button_type, mut background_color) in &mut button_query {
        match interaction {
            Interaction::Pressed => match button_type {
                ButtonType::SetWindowMode(mode) => {
                    window.mode = *mode;
                }
                ButtonType::SetResolution((width, height)) => {
                    window.resolution.set_physical_resolution(*width, *height);
                }
                ButtonType::SetPosition(position) => {
                    window.position = *position;
                }
                ButtonType::SetScaleFactor(scale_factor) => {
                    window.resolution.set_scale_factor(*scale_factor);
                }
                ButtonType::SetPresentMode(mode) => {
                    window.present_mode = *mode;
                }
            },
            Interaction::Hovered => {
                background_color.0 = tailwind::SLATE_600.into();
            }
            Interaction::None => {
                background_color.0 = tailwind::SLATE_700.into();
            }
        }
    }
}

fn create_button(child_builder: &mut ChildBuilder, button_type: ButtonType, text: &str) {
    child_builder
        .spawn((
            button_type,
            Button,
            BackgroundColor::from(tailwind::SLATE_700),
            Node {
                padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
                ..default()
            },
        ))
        .with_child((
            Text::new(text),
            TextFont {
                font_size: 16.0,
                ..default()
            },
        ));
}