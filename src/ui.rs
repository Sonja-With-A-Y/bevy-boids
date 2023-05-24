use crate::*;

pub fn ui(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
    .with_children(|parent| {
        // text
        parent.spawn((
            TextBundle::from_section(
                "Text Example",
                TextStyle {
                    font: game_assets.ui_font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            }),
            // Because this is a distinct label widget and
            // not button/list item text, this is necessary
            // for accessibility to treat the text accordingly.
            Label,
        ));
    });
}

