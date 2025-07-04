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
        .add_systems(
            Update,
            (
                update_ui,
                update_buttons,
                update_monitor_buttons,
                update_pending,
            ),
        )
        .init_resource::<PendingChanges>()
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
    WindowMode,
    PresentMode,
    PhysicalWindowSize,
    LogicalWindowSize,
    ScaleFactor,
    WindowPosition,
    Monitor,
}

#[derive(Component)]
enum ButtonType {
    Apply,
    Cancel,
    SetWindowMode(WindowMode),
    SetPresentMode(PresentMode),
    SetResolution((u32, u32)),
    SetScaleFactor(Option<f32>),
    SetPosition(WindowPosition),
}

const ACTIVE_COLOR: Srgba = tailwind::BLUE_300;
const HOVER_COLOR: Srgba = tailwind::SLATE_700;
const BUTTON_COLOR: Srgba = tailwind::SLATE_600;

impl ButtonType {
    fn is_active(&self, pending: &PendingChanges) -> bool {
        match self {
            ButtonType::Apply => false,
            ButtonType::Cancel => false,
            ButtonType::SetWindowMode(mode) => pending.window_mode == Some(*mode),
            ButtonType::SetPresentMode(mode) => pending.present_mode == Some(*mode),
            ButtonType::SetResolution((width, height)) => {
                pending.resolution == Some((*width, *height))
            }
            ButtonType::SetScaleFactor(scale_factor) => pending.scale_factor == Some(*scale_factor),
            ButtonType::SetPosition(position) => pending.position == Some(*position),
        }
    }

    fn get_color(&self, pending: &PendingChanges, hovering: bool) -> Color {
        match (self, self.is_active(&pending), hovering) {
            (_, true, _) => ACTIVE_COLOR.into(),
            (ButtonType::Apply | ButtonType::Cancel, false, _) if pending.is_empty() => {
                BUTTON_COLOR.with_alpha(0.1).into()
            }
            (_, _, true) => HOVER_COLOR.into(),
            _ => BUTTON_COLOR.into(),
        }
    }
}

#[derive(Default, Resource)]
pub struct PendingChanges {
    pub window_mode: Option<WindowMode>,
    pub present_mode: Option<PresentMode>,
    pub resolution: Option<(u32, u32)>,
    pub scale_factor: Option<Option<f32>>,
    pub position: Option<WindowPosition>,
}

impl PendingChanges {
    pub fn is_empty(&self) -> bool {
        self.window_mode.is_none()
            && self.present_mode.is_none()
            && self.resolution.is_none()
            && self.scale_factor.is_none()
            && self.position.is_none()
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let font = TextFont {
        font_size: 14.0,
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
                    create_button(parent, ButtonType::Apply, "Apply");
                    create_button(parent, ButtonType::Cancel, "Cancel");
                });

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
                        ButtonType::SetWindowMode(WindowMode::Fullscreen(
                            MonitorSelection::Current,
                            VideoModeSelection::Current,
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
                    create_button(
                        parent,
                        ButtonType::SetPresentMode(PresentMode::Fifo),
                        "Set Fifo",
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
                    create_button(
                        parent,
                        ButtonType::SetResolution((2560, 1440)),
                        "Set 2560x1440",
                    );
                });

            parent
                .spawn(Node {
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    create_button(parent, ButtonType::SetScaleFactor(None), "Reset scaling");
                    create_button(
                        parent,
                        ButtonType::SetScaleFactor(Some(1.0)),
                        "Set 1x scaling",
                    );
                    create_button(
                        parent,
                        ButtonType::SetScaleFactor(Some(1.5)),
                        "Set 1.5x scaling",
                    );
                    create_button(
                        parent,
                        ButtonType::SetScaleFactor(Some(2.0)),
                        "Set 2x scaling",
                    );
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
                .with_child((TextMode::PhysicalWindowSize, Text::new(""), font.clone()))
                .with_child((TextMode::LogicalWindowSize, Text::new(""), font.clone()))
                .with_child((TextMode::ScaleFactor, Text::new(""), font.clone()))
                .with_child((TextMode::WindowPosition, Text::new(""), font.clone()))
                .with_child((TextMode::Monitor, Text::new(""), font.clone()));
        });
}

fn update_monitor_buttons(
    mut commands: Commands,
    monitor_node: Single<(Entity, Option<&Children>), With<MonitorNode>>,
    monitors: Query<&Monitor>,
    mut monitor_len: Local<usize>,
) {
    if *monitor_len == monitors.iter().len() {
        return;
    }

    *monitor_len = monitors.iter().len();

    if let Some(children) = monitor_node.1 {
        for child in children.iter() {
            commands.entity(child).despawn();
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
            TextMode::ScaleFactor => match window.resolution.scale_factor_override() {
                None => format!("Scale factor: {}", window.resolution.base_scale_factor()),
                Some(scale_override) => {
                    format!(
                        "Scale factor: {} (Override: {})",
                        window.resolution.base_scale_factor(),
                        scale_override
                    )
                }
            },
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
    mut pending: ResMut<PendingChanges>,
    mut button_query: Query<
        (&Interaction, &ButtonType, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut window: Single<&mut Window>,
    monitors: Query<&Monitor>,
) {
    for (interaction, button_type, mut background_color) in &mut button_query {
        match interaction {
            Interaction::Pressed => match button_type {
                ButtonType::Apply => {
                    if let Some(mode) = pending.window_mode.take() {
                        window.mode = mode;
                    }

                    match window.mode {
                        WindowMode::BorderlessFullscreen(_) => {
                            // Set pending.position to None and handle separately if in Borderless
                            if let Some(WindowPosition::Centered(monitor_selection)) =
                                pending.position
                            {
                                window.mode = WindowMode::BorderlessFullscreen(monitor_selection);
                                pending.position = None;
                            }
                        }
                        WindowMode::Fullscreen(current_monitor, current_videomode) => {
                            let monitor_selection =
                                if let Some(WindowPosition::Centered(monitor_selection)) =
                                    pending.position
                                {
                                    pending.position = None;

                                    monitor_selection
                                } else {
                                    current_monitor
                                };

                            let video_mode = if let Some((width, height)) = pending.resolution {
                                pending.resolution = None;

                                // TODO: Try to work this out properly as it makes a lot of assumptions.
                                VideoModeSelection::Specific(bevy::window::VideoMode {
                                    physical_size: UVec2::new(width, height),
                                    bit_depth: 32,
                                    refresh_rate_millihertz: monitors
                                        .iter()
                                        .flat_map(|m| m.video_modes.iter())
                                        .max_by_key(|vm| vm.refresh_rate_millihertz)
                                        .unwrap()
                                        .refresh_rate_millihertz,
                                })
                            } else {
                                current_videomode
                            };

                            window.mode = WindowMode::Fullscreen(monitor_selection, video_mode);
                        }
                        _ => {}
                    }

                    if let Some(mode) = pending.present_mode.take() {
                        window.present_mode = mode;
                    }
                    if let Some((width, height)) = pending.resolution.take() {
                        window.resolution.set_physical_resolution(width, height);
                    }
                    if let Some(scale_factor) = pending.scale_factor.take() {
                        window.resolution.set_scale_factor_override(scale_factor);
                    }
                    if let Some(position) = pending.position.take() {
                        window.position = position;
                    }

                    *pending = PendingChanges::default();
                }
                ButtonType::Cancel => {
                    *pending = PendingChanges::default();
                }
                ButtonType::SetWindowMode(mode) => {
                    pending.window_mode = Some(*mode);
                }
                ButtonType::SetPresentMode(mode) => {
                    pending.present_mode = Some(*mode);
                }
                ButtonType::SetResolution((width, height)) => {
                    pending.resolution = Some((*width, *height));
                }
                ButtonType::SetScaleFactor(scale_factor) => {
                    pending.scale_factor = Some(*scale_factor);
                }
                ButtonType::SetPosition(position) => {
                    pending.position = Some(*position);
                }
            },
            Interaction::Hovered => {
                background_color.0 = button_type.get_color(&pending, true);
            }
            Interaction::None => {
                background_color.0 = button_type.get_color(&pending, false);
            }
        }
    }
}

fn update_pending(
    pending: Res<PendingChanges>,
    mut button_query: Query<(&ButtonType, &mut BackgroundColor)>,
) {
    if pending.is_changed() {
        for (button_type, mut background_color) in &mut button_query {
            background_color.0 = button_type.get_color(&pending, false);
        }
    }
}

fn create_button(child_builder: &mut ChildSpawnerCommands, button_type: ButtonType, text: &str) {
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
                font_size: 14.0,
                ..default()
            },
        ));
}
