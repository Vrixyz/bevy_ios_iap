use crate::loading::TextureAssets;
use crate::GameState;
use bevy::ecs::system::{EntityCommand, EntityCommands};
use bevy::prelude::*;

static iap_consumable: &str = "com.thierryberger.bevyiap.testconsumable";
static iap_nonconsumable: &str = "com.thierryberger.bevyiap.testnonconsumable";

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(
                Update,
                click_iap_initialize_button.run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component, Clone)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    info!("menu");
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            create_button(children, "Fetch products", &button_colors).insert(InitializeIAPButton);
            create_button(children, "Buy Consumable", &button_colors)
                .insert(PurchaseIAPButton(iap_consumable.to_string()));
            create_button(children, "Buy Non Consumable", &button_colors)
                .insert(PurchaseIAPButton(iap_nonconsumable.to_string()));
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Made with Bevy",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.bevy.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/NiklasEi/bevy_game_template"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Open source",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.github.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

fn create_button<'a>(
    children: &'a mut ChildBuilder<'_>,
    title: &str,
    button_colors: &ButtonColors,
) -> EntityCommands<'a> {
    let mut entity_commands = children.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(240.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        },
        button_colors.clone(),
    ));
    entity_commands.with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            title,
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        ));
    });
    entity_commands
}

#[derive(Component)]
struct InitializeIAPButton;

#[derive(Component)]
struct PurchaseIAPButton(pub String);

#[derive(Component)]
struct OpenLink(&'static str);

fn click_iap_initialize_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&InitializeIAPButton>,
            Option<&PurchaseIAPButton>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, initialize_iap, purchase, open_link) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = initialize_iap {
                    let product_ids =
                        vec![iap_consumable.to_string(), iap_nonconsumable.to_string()];
                    dbg!("initializing iap...", &product_ids);
                    bevy_ios_iap::fetch_products_for_identifiers(product_ids);
                } else if let Some(purchase) = purchase {
                    // TODO: check if initialized
                    if bevy_ios_iap::can_purchase(&purchase.0) {
                        dbg!("buying:", &purchase.0);
                        dbg!("price: ", bevy_ios_iap::get_price_localized(&purchase.0));
                        bevy_ios_iap::purchase(&purchase.0);
                    } else {
                        dbg!("cannot purchase this product currently.");
                    }
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
