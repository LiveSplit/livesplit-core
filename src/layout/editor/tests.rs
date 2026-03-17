use crate::{
    Lang,
    component::{group, separator, text, timer},
    layout::{Component, Layout, LayoutDirection, editor::Editor},
    settings::ImageCache,
};

fn two_component_layout() -> Layout {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    layout.push(text::Component::new());
    layout
}

fn layout_with_group() -> Layout {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    let mut group = group::Component::new();
    group
        .components
        .push(Component::from(text::Component::new()));
    group
        .components
        .push(Component::from(separator::Component::new()));
    layout.push(group);
    layout.push(text::Component::new());
    layout
}

// ---- Basic editor creation ----

#[test]
fn create_editor_with_group() {
    let layout = layout_with_group();
    let _editor = Editor::new(layout).unwrap();
}

#[test]
fn empty_layout_fails() {
    let layout = Layout::new();
    assert!(Editor::new(layout).is_err());
}

// ---- Flat view shows all components with indent levels ----

#[test]
fn state_shows_flat_view_with_indentation() {
    let layout = layout_with_group();
    let editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Layout: Timer, Group(Text, Separator), Text
    // Flat: Timer(0), Group(0), Text(1), Separator(1), Text(0)
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.components.len(), 5);
    assert_eq!(state.indent_levels.len(), 5);
    assert_eq!(state.indent_levels[0], 0); // Timer
    assert_eq!(state.indent_levels[1], 0); // Group
    assert_eq!(state.indent_levels[2], 1); // Text inside group
    assert_eq!(state.indent_levels[3], 1); // Separator inside group
    assert_eq!(state.indent_levels[4], 0); // Text
    // None of these are placeholders.
    assert!(state.is_placeholder.iter().all(|&p| !p));
}

// ---- Add component ----

#[test]
fn add_component_at_top_level() {
    let layout = two_component_layout();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select first component and add after it.
    editor.select(0);
    editor.add_component(separator::Component::new());
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.components.len(), 3);
    assert_eq!(state.selected_component, 1); // New component selected
    assert_eq!(state.indent_levels[1], 0); // At top level
}

#[test]
fn add_component_inside_group() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select a child inside the group (flat index 2 = Text inside group).
    editor.select(2);
    editor.add_component(separator::Component::new());
    let state = editor.state(&mut image_cache, Lang::English);
    // Was 5 (Timer, Group, Text, Sep, Text), now 6.
    assert_eq!(state.components.len(), 6);
    // New component is selected, at indent level 1 (inside the group).
    assert_eq!(state.indent_levels[state.selected_component as usize], 1);
}

#[test]
fn add_component_to_selected_group_adds_after() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select the group itself (flat index 1).
    editor.select(1);
    editor.add_component(separator::Component::new());
    let state = editor.state(&mut image_cache, Lang::English);
    // New component inserted after the group (and its children), at top level.
    // Flat: Timer(0), Group(0), Text(1), Sep(1), [NEW Sep](0), Text(0)
    assert_eq!(state.components.len(), 6);
    let sel = state.selected_component as usize;
    assert_eq!(state.indent_levels[sel], 0); // At top level, not inside group
}

#[test]
fn add_group_via_add_component() {
    let layout = two_component_layout();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    editor.add_component(group::Component::new());
    let state = editor.state(&mut image_cache, Lang::English);
    // Timer, Row, [Empty placeholder], Text
    assert_eq!(state.components.len(), 4);
    // The placeholder is automatically selected.
    let sel = state.selected_component as usize;
    assert_eq!(sel, 2);
    assert_eq!(&state.components[sel], "Empty");
    assert!(state.is_placeholder[sel]);
    assert_eq!(state.indent_levels[sel], 1);
}

#[test]
fn add_group_nesting() {
    let layout = two_component_layout();
    let mut editor = Editor::new(layout).unwrap();

    // Add a group at top level. Its placeholder is automatically selected.
    editor.add_component(group::Component::new());

    // Add a nested group inside the first group (placeholder is selected,
    // so the new group becomes the first child). Its placeholder is
    // automatically selected.
    editor.add_component(group::Component::new());

    // Add a third-level group (same mechanism).
    editor.add_component(group::Component::new());

    let layout = editor.close();

    // Top-level group (index 1).
    let outer = match &layout.components[1] {
        Component::Group(g) => g,
        _ => panic!("Expected outer group"),
    };

    // Middle group (first child of outer).
    let middle = match &outer.components[0] {
        Component::Group(g) => g,
        _ => panic!("Expected middle group"),
    };

    // Inner group (first child of middle).
    match &middle.components[0] {
        Component::Group(_) => {}
        _ => panic!("Expected inner group"),
    };
}

#[test]
fn layout_direction_alternates_with_depth() {
    let layout = two_component_layout();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Top level in a vertical layout → direction is Vertical.
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.layout_direction, LayoutDirection::Vertical);

    // Add a group. Its placeholder is auto-selected (indent 1).
    editor.add_component(group::Component::new());
    let state = editor.state(&mut image_cache, Lang::English);
    // Inside a group at depth 1 → direction flips to Horizontal.
    assert_eq!(state.layout_direction, LayoutDirection::Horizontal);

    // Add another group inside. Its placeholder is auto-selected (indent 2).
    editor.add_component(group::Component::new());
    let state = editor.state(&mut image_cache, Lang::English);
    // Depth 2 → direction flips back to Vertical.
    assert_eq!(state.layout_direction, LayoutDirection::Vertical);
}

// ---- Remove component ----

#[test]
fn remove_top_level_component() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select Timer (flat 0), remove it.
    editor.select(0);
    editor.remove_component();
    let state = editor.state(&mut image_cache, Lang::English);
    // Was 5, now 4 (Group, Text, Sep, Text).
    assert_eq!(state.components.len(), 4);
}

#[test]
fn remove_group_removes_children() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select the group (flat 1), remove it. The group and its children go away.
    editor.select(1);
    editor.remove_component();
    let state = editor.state(&mut image_cache, Lang::English);
    // Was 5, lost Group + 2 children = 2 remaining (Timer, Text).
    assert_eq!(state.components.len(), 2);
}

#[test]
fn remove_child_inside_group() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select Text inside group (flat 2), remove it.
    editor.select(2);
    editor.remove_component();
    let state = editor.state(&mut image_cache, Lang::English);
    // Was 5, now 4 (Timer, Group, Separator, Text).
    assert_eq!(state.components.len(), 4);
}

#[test]
fn remove_last_child_selects_group() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    let mut group = group::Component::new();
    group
        .components
        .push(Component::from(text::Component::new()));
    layout.push(group);
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Flat: Timer(0), Group(0), Text(1)
    // Select the only child (flat 2) and remove it.
    editor.select(2);
    editor.remove_component();
    let state = editor.state(&mut image_cache, Lang::English);
    // Now: Timer(0), Row(0), Empty(1) — group is empty with placeholder.
    assert_eq!(state.components.len(), 3);
    // The group itself should be selected.
    assert_eq!(state.selected_component, 1);
    assert_eq!(&state.components[1], "Row");
    // Placeholder follows.
    assert_eq!(&state.components[2], "Empty");
    assert_eq!(state.indent_levels[2], 1);
    assert!(state.is_placeholder[2]);
}

#[test]
fn cannot_remove_last_top_level_component() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    let editor = Editor::new(layout).unwrap();
    assert!(!editor.can_remove_component());
}

#[test]
fn can_remove_child_even_if_its_the_only_child() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    let mut group = group::Component::new();
    group
        .components
        .push(Component::from(text::Component::new()));
    layout.push(group);
    let mut editor = Editor::new(layout).unwrap();

    // Select the only child inside the group (flat 2).
    editor.select(2);
    assert!(editor.can_remove_component());
}

// ---- Move component ----

#[test]
fn move_component_up_at_top_level() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select the last top-level component (Text at flat 4).
    editor.select(4);
    assert!(editor.can_move_component_up());
    editor.move_component_up();
    let state = editor.state(&mut image_cache, Lang::English);
    // The Text should now be at top-level index 1 (before the group).
    // Flat: Timer(0), Text(0), Group(0), Text(1), Sep(1)
    assert_eq!(state.indent_levels[1], 0);
}

#[test]
fn move_component_down_inside_group() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select the first child inside the group (flat 2 = Text).
    editor.select(2);
    assert!(editor.can_move_component_down());
    editor.move_component_down();
    let state = editor.state(&mut image_cache, Lang::English);
    // The Text and Separator inside the group should be swapped.
    // Now: Timer(0), Group(0), Separator(1), Text(1), Text(0)
    assert_eq!(state.selected_component, 3); // Moved text is now at flat 3
    assert_eq!(state.indent_levels[3], 1);
}

#[test]
fn cannot_move_first_sibling_up() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();

    // Select the first child inside the group (flat 2).
    editor.select(2);
    assert!(!editor.can_move_component_up());
}

#[test]
fn cannot_move_last_sibling_down() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();

    // Select the last child inside the group (flat 3 = Separator).
    editor.select(3);
    assert!(!editor.can_move_component_down());
}

// ---- Duplicate component ----

#[test]
fn duplicate_top_level_component() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    editor.select(0); // Timer
    assert!(editor.can_duplicate_component());
    let state = editor.state(&mut image_cache, Lang::English);
    assert!(state.buttons.can_duplicate);
    editor.duplicate_component();
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.components.len(), 6); // Was 5
    assert_eq!(state.selected_component, 1); // Duplicate selected
}

#[test]
fn duplicate_group_duplicates_children() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    editor.select(1); // Group
    editor.duplicate_component();
    let state = editor.state(&mut image_cache, Lang::English);
    // Was 5, group has 2 children, so duplicating adds 3 (group + 2 children) = 8 total.
    assert_eq!(state.components.len(), 8);
}

#[test]
fn duplicate_child_inside_group() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    editor.select(2); // Text inside group
    editor.duplicate_component();
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.components.len(), 6); // Was 5
    assert_eq!(state.indent_levels[state.selected_component as usize], 1);
}

// ---- Component settings ----

#[test]
fn settings_for_child_inside_group() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select a child inside the group.
    editor.select(2);
    let state = editor.state(&mut image_cache, Lang::English);
    assert!(!state.component_settings.fields.is_empty());
}

#[test]
fn settings_for_group_itself() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select the group.
    editor.select(1);
    let state = editor.state(&mut image_cache, Lang::English);
    // Group has settings (direction, size).
    assert!(!state.component_settings.fields.is_empty());
}

// ---- Close preserves structure ----

#[test]
fn close_preserves_group() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();

    // Select a child inside the group and add a component.
    editor.select(2);
    editor.add_component(separator::Component::new());

    let layout = editor.close();
    match &layout.components[1] {
        Component::Group(group) => {
            assert_eq!(group.components.len(), 3); // 2 original + 1 added
        }
        _ => panic!("Expected a group at index 1"),
    }
}

// ---- Nested groups ----

#[test]
fn nested_group_flat_view() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());

    let mut inner_group = group::Component::new();
    inner_group
        .components
        .push(Component::from(text::Component::new()));

    let mut outer_group = group::Component::new();
    outer_group
        .components
        .push(Component::from(separator::Component::new()));
    outer_group.components.push(Component::Group(inner_group));

    layout.push(outer_group);

    let editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);

    // Flat: Timer(0), OuterGroup(0), Separator(1), InnerGroup(1), Text(2)
    assert_eq!(state.components.len(), 5);
    assert_eq!(state.indent_levels[0], 0); // Timer
    assert_eq!(state.indent_levels[1], 0); // Outer group
    assert_eq!(state.indent_levels[2], 1); // Separator in outer
    assert_eq!(state.indent_levels[3], 1); // Inner group in outer
    assert_eq!(state.indent_levels[4], 2); // Text in inner
}

#[test]
fn select_deeply_nested_component() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());

    let mut inner_group = group::Component::new();
    inner_group
        .components
        .push(Component::from(text::Component::new()));

    let mut outer_group = group::Component::new();
    outer_group.components.push(Component::Group(inner_group));

    layout.push(outer_group);

    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Flat: Timer(0), OuterGroup(0), InnerGroup(1), Text(2)
    editor.select(3); // Text at depth 2
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.selected_component, 3);
    assert_eq!(state.indent_levels[3], 2);
}

// ---- Empty group placeholder ----

#[test]
fn empty_group_shows_placeholder() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    layout.push(group::Component::new());
    layout.push(text::Component::new());

    let editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);

    // Flat: Timer(0), Group(0), Empty(1), Text(0)
    assert_eq!(state.components.len(), 4);
    assert_eq!(&state.components[2], "Empty");
    assert_eq!(state.indent_levels[2], 1);
    // Only the placeholder at index 2 is marked.
    assert!(!state.is_placeholder[0]);
    assert!(!state.is_placeholder[1]);
    assert!(state.is_placeholder[2]);
    assert!(!state.is_placeholder[3]);
}

#[test]
fn placeholder_not_removable() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    layout.push(group::Component::new());

    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select the placeholder (flat index 2).
    editor.select(2);
    assert!(!editor.can_remove_component());
    let state = editor.state(&mut image_cache, Lang::English);
    assert!(!state.buttons.can_remove);
}

#[test]
fn placeholder_not_movable() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    layout.push(group::Component::new());

    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    editor.select(2); // Placeholder
    assert!(!editor.can_move_component_up());
    assert!(!editor.can_move_component_down());
    let state = editor.state(&mut image_cache, Lang::English);
    assert!(!state.buttons.can_move_up);
    assert!(!state.buttons.can_move_down);
}

#[test]
fn placeholder_not_duplicable() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    layout.push(group::Component::new());

    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    editor.select(2); // Placeholder
    assert!(!editor.can_duplicate_component());
    let state = editor.state(&mut image_cache, Lang::English);
    assert!(!state.buttons.can_duplicate);
    editor.duplicate_component();
    let state = editor.state(&mut image_cache, Lang::English);
    // No change – still 3 entries.
    assert_eq!(state.components.len(), 3);
}

#[test]
fn placeholder_has_empty_settings() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    layout.push(group::Component::new());

    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    editor.select(2); // Placeholder
    let state = editor.state(&mut image_cache, Lang::English);
    assert!(state.component_settings.fields.is_empty());
}

#[test]
fn add_component_to_placeholder_adds_inside_group() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    layout.push(group::Component::new());

    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Flat: Timer(0), Group(0), Empty(1)
    editor.select(2); // Placeholder
    editor.add_component(separator::Component::new());
    let state = editor.state(&mut image_cache, Lang::English);

    // Placeholder should be gone, component added inside group.
    // Flat: Timer(0), Group(0), Separator(1)
    assert_eq!(state.components.len(), 3);
    assert_eq!(state.indent_levels[2], 1);
    assert_eq!(&state.components[2], "Separator");
    assert_eq!(state.selected_component, 2);
}

// ---- Cross-group move_component ----

#[test]
fn move_component_into_group() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Flat: Timer(0), Group(0), Text(1), Separator(1), Text(0)
    // Move Timer (flat 0) to where Text inside group is (flat 2).
    editor.select(0);
    editor.move_component(2);
    let state = editor.state(&mut image_cache, Lang::English);

    // Timer should now be inside the group (after Text).
    // Flat: Group(0), Text(1), Timer(1), Separator(1), Text(0)
    assert_eq!(state.components.len(), 5);
    assert_eq!(state.indent_levels[state.selected_component as usize], 1);
}

#[test]
fn move_component_out_of_group() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Flat: Timer(0), Group(0), Text(1), Separator(1), Text(0)
    // Move Text inside group (flat 2) to where Timer is (flat 0).
    editor.select(2);
    editor.move_component(0);
    let state = editor.state(&mut image_cache, Lang::English);

    // Text should now be at top level before Timer.
    assert_eq!(state.indent_levels[state.selected_component as usize], 0);
    assert_eq!(state.selected_component, 0);
}

#[test]
fn move_component_across_groups() {
    // Create layout with two groups.
    let mut layout = Layout::new();
    let mut group_a = group::Component::new();
    group_a
        .components
        .push(Component::from(timer::Component::new()));
    let mut group_b = group::Component::new();
    group_b
        .components
        .push(Component::from(text::Component::new()));
    layout.push(group_a);
    layout.push(group_b);

    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Flat: GroupA(0), Timer(1), GroupB(0), Text(1)
    // Move Timer (flat 1) to where Text is (flat 3).
    editor.select(1);
    editor.move_component(3);
    let state = editor.state(&mut image_cache, Lang::English);

    // Timer should now be inside GroupB, after Text.
    // GroupA is now empty → has placeholder.
    // Flat: GroupA(0), Empty(1), GroupB(0), Text(1), Timer(1)
    assert_eq!(state.components.len(), 5);
    assert_eq!(&state.components[1], "Empty");
    assert!(state.is_placeholder[1]);
    assert_eq!(state.indent_levels[4], 1); // Timer inside GroupB
}

#[test]
fn move_component_to_placeholder() {
    // Create layout with an empty group.
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    layout.push(group::Component::new());

    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Flat: Timer(0), Group(0), Empty(1)
    // Move Timer (flat 0) to the placeholder (flat 2).
    editor.select(0);
    editor.move_component(2);
    let state = editor.state(&mut image_cache, Lang::English);

    // Timer should now be inside the group.
    // Flat: Group(0), Timer(1)
    assert_eq!(state.components.len(), 2);
    assert_eq!(state.indent_levels[1], 1);
    assert_eq!(&state.components[1], "Timer");
}

#[test]
fn move_component_to_same_position_is_noop() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    editor.select(0);
    editor.move_component(0);
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.selected_component, 0);
    assert_eq!(state.components.len(), 5);
}

#[test]
fn move_group_with_children_down() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Flat: Timer(0), Group(0), Text(1), Separator(1), Text(0)
    // Move the Group (flat 1, subtree 1-3) past the last Text (flat 4).
    editor.select(1);
    editor.move_component(4);
    let state = editor.state(&mut image_cache, Lang::English);

    // Flat: Timer(0), Text(0), Group(0), Text(1), Separator(1)
    assert_eq!(state.components.len(), 5);
    assert_eq!(&state.components[0], "Timer");
    assert_eq!(state.indent_levels[0], 0);
    assert_eq!(state.indent_levels[1], 0); // Text moved to position 1
    assert_eq!(&state.components[2], "Row");
    assert_eq!(state.indent_levels[2], 0);
}

#[test]
fn cannot_move_into_own_subtree() {
    let layout = layout_with_group();
    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select the Group (flat 1), try to move to its child (flat 2).
    editor.select(1);
    editor.move_component(2);
    let state = editor.state(&mut image_cache, Lang::English);
    // Should be a no-op.
    assert_eq!(state.selected_component, 1);
    assert_eq!(state.components.len(), 5);
}

#[test]
fn cannot_move_placeholder() {
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    layout.push(group::Component::new());

    let mut editor = Editor::new(layout).unwrap();
    let mut image_cache = ImageCache::new();

    // Select the placeholder (flat 2) and try to move it.
    editor.select(2);
    editor.move_component(0);
    let state = editor.state(&mut image_cache, Lang::English);
    // Should be a no-op – placeholders can't be moved.
    assert_eq!(state.selected_component, 2);
    assert_eq!(state.components.len(), 3);
}
