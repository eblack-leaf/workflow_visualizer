use crate::resources::ResourceHandles;
use std::collections::{HashMap, HashSet};
use workflow_visualizer::bevy_ecs::entity::Entity;
use workflow_visualizer::bevy_ecs::prelude::Resource;
use workflow_visualizer::bevy_ecs::system::{Commands, Query, Res, ResMut};
use workflow_visualizer::{
    bevy_ecs, BundlePlacement, Button, ButtonBorder, ButtonType, Color, Disabled, Grid, GridView,
    GridViewBuilder, Key, KeyFactory, KnownTextDimension, Layer, ListEntryDescriptor,
    MonoSpacedFont, Panel, PanelType, RawMarker, Triggered,
};
use workflow_visualizer::{GridPoint, List, ResponsiveUnit};

#[derive(Resource)]
pub(crate) struct NodePanel {
    pub(crate) node_list: List<Key>,
    pub(crate) key_factory: KeyFactory,
    pub(crate) node_entries: HashMap<Key, NodeEntry>,
    add: Option<Entity>,
    page_right: Option<Entity>,
    page_left: Option<Entity>,
}
impl NodePanel {
    pub(crate) const NODE_ENTRY_HEIGHT: RawMarker = RawMarker(15);
    pub(crate) const NODE_ENTRY_PADDING: RawMarker = RawMarker(2);
    pub(crate) const NODE_LIST_LAYER: Layer = Layer { z: 5f32 };
    pub(crate) const NODE_LIST_RIGHT: i32 = 2;
    pub(crate) const NODE_LIST_BOTTOM: i32 = 6;
}
#[derive(Resource)]
pub(crate) struct NodeEditor {
    view: GridView,
}
pub(crate) struct NodeEntry {
    pub(crate) selector: Option<Entity>,
    pub(crate) deleter: Option<Entity>,
    pub(crate) panel: Option<Entity>,
}
impl NodeEntry {
    pub(crate) fn disable(&self, cmd: &mut Commands) {
        if let Some(entity) = self.selector.as_ref() {
            cmd.entity(*entity).insert(Disabled::default());
        }
        if let Some(entity) = self.deleter.as_ref() {
            cmd.entity(*entity).insert(Disabled::default());
        }
        if let Some(entity) = self.panel.as_ref() {
            cmd.entity(*entity).insert(Disabled::default());
        }
    }
    pub(crate) fn enable(&self, cmd: &mut Commands) {
        if let Some(entity) = self.selector.as_ref() {
            cmd.entity(*entity).remove::<Disabled>();
        }
        if let Some(entity) = self.deleter.as_ref() {
            cmd.entity(*entity).remove::<Disabled>();
        }
        if let Some(entity) = self.panel.as_ref() {
            cmd.entity(*entity).remove::<Disabled>();
        }
    }
}
pub(crate) fn place_node_entry(
    name: String,
    position: GridPoint,
    cmd: &mut Commands,
    font: &MonoSpacedFont,
    desc: &ListEntryDescriptor,
) -> NodeEntry {
    let scale = font.text_scale_from_dimension(KnownTextDimension::Height(
        (NodePanel::NODE_ENTRY_HEIGHT - RawMarker(2)).to_pixel() as u32,
    ));
    let panel_view = GridViewBuilder::new()
        .with_top(position.y)
        .with_left(position.x)
        .with_right(position.x.raw_offset(desc.width))
        .with_bottom(position.y.raw_offset(desc.height))
        .build();
    let panel = cmd
        .spawn(
            Panel::new(
                PanelType::Flat,
                NodePanel::NODE_LIST_LAYER,
                Color::GREY_DARK,
                Color::GREY_DARK,
            )
            .responsively_viewed(panel_view.all_same()),
        )
        .id();
    let deleter_size = scale.0 as i32 + 2;
    let split_location = panel_view.right().raw_offset(-deleter_size);
    let selector = cmd
        .spawn(
            Button::new(
                ButtonType::Toggle,
                NodePanel::NODE_LIST_LAYER,
                Color::OFF_WHITE,
                Color::OFF_BLACK,
                ResourceHandles::NodeIcon.handle(),
                name,
                scale,
                0,
                ButtonBorder::Border,
            )
            .responsively_viewed(
                GridViewBuilder::from(panel_view)
                    .with_right(split_location)
                    .build()
                    .all_same(),
            ),
        )
        .id();
    let deleter = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                NodePanel::NODE_LIST_LAYER,
                Color::RED,
                Color::RED_DARK,
                ResourceHandles::NodeIcon.handle(),
                "",
                0,
                scale.0,
                ButtonBorder::None,
            )
            .responsively_viewed(
                GridViewBuilder::from(panel_view)
                    .with_left(split_location)
                    .build()
                    .all_same(),
            ),
        )
        .id();
    NodeEntry {
        panel: Some(panel),
        selector: Some(selector),
        deleter: Some(deleter),
    }
}
#[derive(Resource)]
pub(crate) struct SelectedNode(pub(crate) Option<Key>);
pub(crate) fn process_triggers_node_panel(
    triggers: Query<&Triggered>,
    mut node_panel: ResMut<NodePanel>,
    mut selected: ResMut<SelectedNode>,
) {
    if let Some(entity) = node_panel.add.as_ref() {
        if let Ok(trigger) = triggers.get(*entity) {
            if trigger.active() {
                let key = node_panel.key_factory.generate();
                node_panel.node_list.add(key);
            }
        }
    }
    let mut removed = HashSet::new();
    for (key, entry) in node_panel.node_entries.iter() {
        if let Some(entity) = entry.selector.as_ref() {
            if let Ok((trigger)) = triggers.get(*entity) {
                if trigger.active() {
                    selected.0.replace(*key);
                }
            }
        }
        if let Some(entity) = entry.deleter.as_ref() {
            if let Ok((trigger)) = triggers.get(*entity) {
                if trigger.active() {
                    removed.insert(*key);
                }
            }
        }
    }
    for node in removed {
        node_panel.node_list.remove(node);
        node_panel.node_entries.remove(&node);
    }
}
pub(crate) fn list_management(mut node_panel: ResMut<NodePanel>, mut cmd: Commands) {
    for enable in node_panel.node_list.enablement() {
        let node_entry = node_panel.node_entries.get(&enable.0);
        if let Some(entry) = node_entry {
            if !enable.1 {
                entry.disable(&mut cmd);
            } else {
                entry.enable(&mut cmd);
            }
        }
    }
}
pub(crate) fn node_panel(mut cmd: Commands, grid: Res<Grid>) {
    let button_panel_offset = 10;
    let list_top = 1.near().raw_offset(button_panel_offset);
    let view = GridViewBuilder::new()
        .with_top(list_top)
        .with_left(1.near())
        .with_right(NodePanel::NODE_LIST_RIGHT.far())
        .with_bottom(NodePanel::NODE_LIST_BOTTOM.far())
        .build();
    let page_left_view = GridViewBuilder::new().build();
    let control_panel_height =
        grid.calc_vertical_location(list_top) - NodePanel::NODE_ENTRY_PADDING;
    let icon_scale_marker = (control_panel_height - RawMarker(2)).0 as u32;
    let page_left = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                NodePanel::NODE_LIST_LAYER,
                Color::OFF_WHITE,
                Color::OFF_BLACK,
                ResourceHandles::NodeIcon.handle(),
                "",
                0,
                icon_scale_marker,
                ButtonBorder::None,
            )
            .responsively_viewed(page_left_view.all_same()),
        )
        .id();
    let page_right_view = GridViewBuilder::new().build();
    let page_right = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                NodePanel::NODE_LIST_LAYER,
                Color::OFF_WHITE,
                Color::OFF_BLACK,
                ResourceHandles::NodeIcon.handle(),
                "",
                0,
                icon_scale_marker,
                ButtonBorder::None,
            )
            .responsively_viewed(page_right_view.all_same()),
        )
        .id();
    let add_view = GridViewBuilder::new().build();
    let add = cmd
        .spawn(
            Button::new(
                ButtonType::Press,
                NodePanel::NODE_LIST_LAYER,
                Color::OFF_WHITE,
                Color::OFF_BLACK,
                ResourceHandles::NodeIcon.handle(),
                "",
                0,
                icon_scale_marker,
                ButtonBorder::None,
            )
            .responsively_viewed(add_view.all_same()),
        )
        .id();
    cmd.insert_resource(NodePanel {
        node_list: List::new(
            GridPoint::new(view.left(), view.top()),
            grid.view_horizontal_markers(&view),
            grid.view_vertical_markers(&view),
            NodePanel::NODE_ENTRY_HEIGHT,
            NodePanel::NODE_ENTRY_PADDING,
        ),

        key_factory: KeyFactory::new(),
        node_entries: HashMap::new(),
        page_left: Some(page_left),
        page_right: Some(page_right),
        add: Some(add),
    });
    let editor_view = GridViewBuilder::new()
        .with_top((NodePanel::NODE_LIST_BOTTOM + 1).near())
        .with_left(1.near())
        .with_right(NodePanel::NODE_LIST_RIGHT.far())
        .with_bottom(grid.last_full_row().far())
        .build();
    cmd.insert_resource(NodeEditor { view: editor_view });
}
