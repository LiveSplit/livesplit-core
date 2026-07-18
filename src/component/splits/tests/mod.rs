use super::{
    ColumnSettings, ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith, Component, Settings,
    State, SubsplitDisplayMode,
};
use crate::{
    Lang, Run, Segment, TimeSpan, Timer, TimingMethod,
    comparison::best_segments,
    component::splits::{ColumnKind, TimeColumn, english_settings},
    settings::{Image, ImageCache, SemanticColor, Value},
};

pub mod column;

#[test]
fn zero_visual_split_count_always_shows_all_splits() {
    let mut run = Run::new();
    for _ in 0..32 {
        run.push_segment(Segment::new(""));
    }
    let timer = Timer::new(run).unwrap();
    let layout_settings = Default::default();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        ..english_settings()
    });

    let mut image_cache = ImageCache::new();

    let mut state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    assert_eq!(state.splits.len(), 32);

    component.scroll_down();
    state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    assert_eq!(state.splits.len(), 32);

    component.scroll_down();
    state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    assert_eq!(state.splits.len(), 32);

    component.scroll_up();
    state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    assert_eq!(state.splits.len(), 32);
}

#[test]
fn one_visual_split() {
    let mut run = Run::new();

    run.push_segment(Segment::new("A"));
    run.push_segment(Segment::new("B"));
    run.push_segment(Segment::new("C"));

    let mut timer = Timer::new(run).unwrap();
    let layout_settings = Default::default();
    let mut component = Component::with_settings(Settings {
        always_show_last_split: false,
        split_preview_count: 0,
        visual_split_count: 1,
        ..english_settings()
    });

    let mut image_cache = ImageCache::new();

    let mut state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    assert_eq!(state.splits[0].name, "A");
    assert_eq!(state.splits.len(), 1);

    timer.start().unwrap();
    state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    assert_eq!(state.splits[0].name, "A");
    assert_eq!(state.splits.len(), 1);

    timer.split().unwrap();
    state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    assert_eq!(state.splits[0].name, "B");
    assert_eq!(state.splits.len(), 1);

    timer.split().unwrap();
    state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    assert_eq!(state.splits[0].name, "C");
    assert_eq!(state.splits.len(), 1);

    timer.split().unwrap();
    state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    assert_eq!(state.splits[0].name, "C");
    assert_eq!(state.splits.len(), 1);
}

#[test]
fn negative_segment_times() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    let mut timer = Timer::new(run).unwrap();
    let layout_settings = Default::default();
    let mut component = Component::with_settings(Settings {
        columns: vec![ColumnSettings {
            kind: ColumnKind::Time(TimeColumn {
                start_with: ColumnStartWith::Empty,
                update_with: ColumnUpdateWith::SegmentTime,
                update_trigger: ColumnUpdateTrigger::OnStartingSegment,
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..english_settings()
    });

    timer.start().unwrap();

    // Emulate a negative offset through game time.
    timer.set_current_timing_method(TimingMethod::GameTime);
    timer.initialize_game_time().unwrap();
    timer.pause_game_time().unwrap();
    timer.set_game_time(TimeSpan::from_seconds(-1.0)).unwrap();

    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    assert_eq!(state.splits[0].columns[0].value, "−1.00");
}

#[test]
fn unique_split_indices() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    run.push_segment(Segment::new(""));
    run.push_segment(Segment::new(""));
    run.segment_groups_mut().push_lossy(0, 2, None, 3);
    let timer = Timer::new(run).unwrap();

    let mut component = Component::with_settings(Settings {
        visual_split_count: 20,
        fill_with_blank_space: true,
        // Expanded groups contain both a header and their major segment. This
        // is the case that requires a dedicated visual-row identity.
        subsplit_display_mode: SubsplitDisplayMode::AllGroupsExpanded,
        ..english_settings()
    });

    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    let mut indices = state
        .splits
        .into_iter()
        .map(|s| s.index)
        .collect::<Vec<_>>();

    indices.sort_unstable();

    assert!(indices.windows(2).all(|pair| pair[0] != pair[1]));
}

#[test]
fn default_subsplit_display_mode_shows_hierarchy() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 3, Some("Chapter A".into()), 4);

    let timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "Outro"]
    );
}

#[test]
fn subsplit_display_mode_setting_uses_typed_value() {
    let mut component = Component::with_settings(Settings {
        subsplit_display_mode: SubsplitDisplayMode::AllGroupsExpanded,
        ..english_settings()
    });

    let description = component.settings_description(Lang::English);
    assert!(description.fields.iter().any(|field| {
        matches!(
            &field.value,
            Value::SubsplitDisplayMode(SubsplitDisplayMode::AllGroupsExpanded)
        )
    }));

    component.set_value(
        14,
        Value::SubsplitDisplayMode(SubsplitDisplayMode::CurrentGroupExpanded),
    );
    let description = component.settings_description(Lang::English);
    assert!(description.fields.iter().any(|field| {
        matches!(
            &field.value,
            Value::SubsplitDisplayMode(SubsplitDisplayMode::CurrentGroupExpanded)
        )
    }));
}

#[test]
fn flat_subsplit_state() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 4, Some("Chapter A".into()), 5);

    let timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::Flat,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "A1", "A2", "A End", "Outro"]
    );
    assert!(state.splits.iter().all(|s| !s.is_indented));
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.section_index)
            .collect::<Vec<_>>(),
        [0, 1, 2, 3, 4]
    );
}

#[test]
fn current_group_expanded_subsplit_state() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    let group_icon = Image::new([1, 2, 3].as_slice().into(), Image::ICON);
    run.segment_groups_mut()
        .push_lossy(1, 4, Some("Chapter A".into()), 5);
    run.segment_groups_mut().set_icon(0, group_icon.clone());

    let mut timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    timer.start().unwrap();
    timer.split().unwrap();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "A1", "A2", "A End", "Outro"]
    );
    assert!(!state.splits[1].is_indented);
    assert_eq!(state.splits[1].index, usize::MAX);
    assert_eq!(state.splits[1].icon, *group_icon.id());
    // No segment has an icon. The group-header icon must still reserve the
    // icon column so the renderer does not overlap it with the header name.
    assert!(state.has_icons);
    assert!(state.splits[1].columns.is_empty());
    assert!(state.splits[2].is_indented);
    assert_eq!(state.splits[2].index, 1);
    assert!(state.splits[4].is_indented);
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.section_index)
            .collect::<Vec<_>>(),
        [0, 1, 1, 1, 1, 2]
    );
}

#[test]
fn single_segment_group_subsplit_state() {
    let mut run = Run::new();
    for name in ["Intro", "A", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 2, Some("Chapter A".into()), 3);

    let mut timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    timer.start().unwrap();
    timer.split().unwrap();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "A", "Outro"]
    );
    assert!(!state.splits[1].is_indented);
    assert!(state.splits[2].is_indented);
    assert_eq!(state.splits[1].index, usize::MAX);
    assert_eq!(state.splits[2].index, 1);
}

#[test]
fn all_groups_expanded_subsplit_state() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 4, Some("Chapter A".into()), 5);

    let timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::AllGroupsExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "A1", "A2", "A End", "Outro"]
    );
    assert!(!state.splits[1].is_indented);
    assert_eq!(state.splits[1].index, usize::MAX);
    assert!(state.splits[1].columns.is_empty());
    assert!(state.splits[2].is_indented);
    assert!(state.splits[3].is_indented);
    assert_eq!(state.splits[4].index, 3);
    assert!(state.splits[4].is_indented);
}

#[test]
fn current_group_expanded_starts_collapsed_before_first_split_when_not_running() {
    let mut run = Run::new();
    for name in ["A1", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(0, 2, Some("Chapter A".into()), 3);

    let timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Chapter A", "Outro"]
    );
    assert!(!state.splits.iter().any(|s| s.is_scrolled_to_split));

    component.scroll_down();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Chapter A", "A1", "A End", "Outro"]
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.is_scrolled_to_split)
            .collect::<Vec<_>>(),
        [false, true, false, false]
    );
}

#[test]
fn current_group_expanded_starts_collapsed_after_final_split_when_ended() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 3, Some("Chapter A".into()), 3);

    let mut timer = Timer::new(run).unwrap();
    timer.start().unwrap();
    timer.split().unwrap();
    timer.split().unwrap();
    timer.split().unwrap();

    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A"]
    );
    assert!(!state.splits.iter().any(|s| s.is_scrolled_to_split));

    component.scroll_up();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "A1", "A End"]
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.is_scrolled_to_split)
            .collect::<Vec<_>>(),
        [false, false, false, true]
    );
}
#[test]
fn current_group_expanded_scrolls_between_groups() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End", "B1", "B End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 3, Some("Chapter A".into()), 6);
    run.segment_groups_mut()
        .push_lossy(3, 5, Some("Chapter B".into()), 6);

    let mut timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    timer.start().unwrap();
    timer.split().unwrap();
    component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );
    component.scroll_down();
    component.scroll_down();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "Chapter B", "B1", "B End", "Outro"]
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.is_scrolled_to_split)
            .collect::<Vec<_>>(),
        [false, false, false, true, false, false]
    );
    assert!(state.splits[3].is_indented);
    assert!(state.splits[1].is_current_split);
    assert_eq!(state.splits[1].name, "Chapter A");
}

#[test]
fn current_group_expanded_scrolls_without_current_split() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End", "B1", "B End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 3, Some("Chapter A".into()), 6);
    run.segment_groups_mut()
        .push_lossy(3, 5, Some("Chapter B".into()), 6);

    let timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );
    component.scroll_down();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "Chapter B", "Outro"]
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.is_scrolled_to_split)
            .collect::<Vec<_>>(),
        [true, false, false, false]
    );
    assert!(!state.splits.iter().any(|s| s.is_current_split));

    component.scroll_down();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "A1", "A End", "Chapter B", "Outro"]
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.is_scrolled_to_split)
            .collect::<Vec<_>>(),
        [false, false, true, false, false, false]
    );
    assert!(!state.splits.iter().any(|s| s.is_current_split));

    component.scroll_down();
    component.scroll_down();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "Chapter B", "B1", "B End", "Outro"]
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.is_scrolled_to_split)
            .collect::<Vec<_>>(),
        [false, false, false, true, false, false]
    );
    assert!(!state.splits.iter().any(|s| s.is_current_split));
}
#[test]
fn current_group_expanded_does_not_scroll_before_first_split_while_running() {
    let mut run = Run::new();
    for name in ["A1", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(0, 2, Some("Chapter A".into()), 3);

    let mut timer = Timer::new(run).unwrap();
    timer.start().unwrap();

    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    component.scroll_up();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Chapter A", "A1", "A End", "Outro"]
    );
    assert!(!state.splits.iter().any(|s| s.is_scrolled_to_split));
}
#[test]
fn subsplit_scroll_cursor_resets_when_current_split_changes() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End", "B1", "B End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 3, Some("Chapter A".into()), 6);
    run.segment_groups_mut()
        .push_lossy(3, 5, Some("Chapter B".into()), 6);

    let mut timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    timer.start().unwrap();
    timer.split().unwrap();
    component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );
    component.scroll_down();
    component.scroll_down();
    timer.split().unwrap();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "A1", "A End", "Chapter B", "Outro"]
    );
    assert!(!state.splits.iter().any(|s| s.is_scrolled_to_split));
    assert!(state.splits[3].is_current_split);
}

#[test]
fn flat_and_all_groups_expanded_scroll_normally_without_subsplit_cursor() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End", "B1", "B End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 3, Some("Chapter A".into()), 6);
    run.segment_groups_mut()
        .push_lossy(3, 5, Some("Chapter B".into()), 6);

    let mut timer = Timer::new(run).unwrap();
    timer.start().unwrap();
    timer.split().unwrap();

    for mode in [
        SubsplitDisplayMode::Flat,
        SubsplitDisplayMode::AllGroupsExpanded,
    ] {
        let mut component = Component::with_settings(Settings {
            visual_split_count: 3,
            always_show_last_split: false,
            subsplit_display_mode: mode,
            ..english_settings()
        });
        let mut image_cache = ImageCache::new();

        component.state(
            &mut image_cache,
            &timer.snapshot(),
            &Default::default(),
            Lang::English,
        );
        component.scroll_down();
        let state = component.state(
            &mut image_cache,
            &timer.snapshot(),
            &Default::default(),
            Lang::English,
        );

        assert!(!state.splits.iter().any(|s| s.is_scrolled_to_split));
        assert_eq!(state.splits.len(), 3);
    }
}

#[test]
fn current_group_expanded_closes_other_groups() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 4, Some("Chapter A".into()), 5);

    let mut timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    timer.start().unwrap();
    timer.split().unwrap();
    timer.split().unwrap();
    timer.split().unwrap();
    timer.split().unwrap();
    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "Outro"]
    );
    assert!(!state.splits[1].is_indented);
    assert!(!state.splits[1].is_current_split);
    assert_eq!(state.splits[1].index, usize::MAX);
    assert_eq!(state.splits[1].columns.len(), 2);
    assert!(state.splits[1].columns.iter().any(|c| !c.value.is_empty()));
    assert!(state.splits[2].is_current_split);
    assert_eq!(state.splits[2].index, 4);
}

#[test]
fn closed_group_header_segment_columns_summarize_the_whole_group() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    for (segment, time) in run
        .segments_mut()
        .iter_mut()
        .zip([10.0, 15.0, 20.0, 30.0, 40.0])
    {
        segment.personal_best_split_time_mut()[TimingMethod::GameTime] =
            Some(TimeSpan::from_seconds(time));
    }
    run.segment_groups_mut()
        .push_lossy(1, 4, Some("Chapter A".into()), 5);

    let mut timer = Timer::new(run).unwrap();
    timer.set_current_timing_method(TimingMethod::GameTime);
    timer.start().unwrap();
    timer.initialize_game_time().unwrap();
    timer.pause_game_time().unwrap();
    for time in [10.0, 17.0, 24.0, 36.0] {
        timer.set_game_time(TimeSpan::from_seconds(time)).unwrap();
        timer.split().unwrap();
    }

    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        columns: vec![
            ColumnSettings {
                kind: ColumnKind::Time(TimeColumn {
                    update_with: ColumnUpdateWith::SegmentTime,
                    ..Default::default()
                }),
                ..Default::default()
            },
            ColumnSettings {
                kind: ColumnKind::Time(TimeColumn {
                    update_with: ColumnUpdateWith::SegmentDelta,
                    ..Default::default()
                }),
                ..Default::default()
            },
        ],
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "Outro"]
    );
    assert!(!state.splits[1].is_indented);
    assert_eq!(state.splits[1].columns[0].value, "26.00");
    assert_eq!(state.splits[1].columns[1].value, "+6.0");
}

#[test]
fn closed_group_header_possible_time_save_summarizes_the_whole_group() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    for (segment, (pb_time, best_segment_time)) in run.segments_mut().iter_mut().zip([
        (10.0, 10.0),
        (20.0, 5.0),
        (30.0, 5.0),
        (40.0, 5.0),
        (50.0, 10.0),
    ]) {
        segment.personal_best_split_time_mut()[TimingMethod::GameTime] =
            Some(TimeSpan::from_seconds(pb_time));
        segment.best_segment_time_mut()[TimingMethod::GameTime] =
            Some(TimeSpan::from_seconds(best_segment_time));
    }
    run.segment_groups_mut()
        .push_lossy(1, 4, Some("Chapter A".into()), 5);

    let mut timer = Timer::new(run).unwrap();
    timer.set_current_timing_method(TimingMethod::GameTime);

    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        columns: vec![ColumnSettings {
            kind: ColumnKind::Time(TimeColumn {
                start_with: ColumnStartWith::PossibleTimeSave,
                update_with: ColumnUpdateWith::DontUpdate,
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    // Each of the three grouped segments can save five seconds. The collapsed
    // header must report their combined value, not only the major segment's
    // five-second value.
    assert_eq!(state.splits[1].name, "Chapter A");
    assert_eq!(state.splits[1].columns[0].value, "15.00");
}

#[test]
fn closed_group_header_segment_delta_can_be_a_best_segment() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    for (segment, (pb_time, best_segment_time)) in run.segments_mut().iter_mut().zip([
        (10.0, 10.0),
        (50.0, 10.0),
        (80.0, 10.0),
        (120.0, 10.0),
        (150.0, 10.0),
    ]) {
        segment.personal_best_split_time_mut()[TimingMethod::GameTime] =
            Some(TimeSpan::from_seconds(pb_time));
        segment.best_segment_time_mut()[TimingMethod::GameTime] =
            Some(TimeSpan::from_seconds(best_segment_time));
    }
    run.segment_groups_mut()
        .push_lossy(1, 4, Some("Chapter A".into()), 5);

    let mut timer = Timer::new(run).unwrap();
    timer.set_current_timing_method(TimingMethod::GameTime);
    timer.start().unwrap();
    timer.initialize_game_time().unwrap();
    timer.pause_game_time().unwrap();
    for time in [10.0, 17.0, 24.0, 36.0] {
        timer.set_game_time(TimeSpan::from_seconds(time)).unwrap();
        timer.split().unwrap();
    }

    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        columns: vec![ColumnSettings {
            kind: ColumnKind::Time(TimeColumn {
                update_with: ColumnUpdateWith::SegmentDelta,
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state.splits[1].columns[0].semantic_color,
        SemanticColor::BestSegment
    );
}

#[test]
fn closed_group_header_split_delta_is_not_best_if_only_the_major_split_is_best() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "Chapter B", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    for (segment, best_segment_time) in run
        .segments_mut()
        .iter_mut()
        .zip([1.0, 1.0, 2.0, 3.0, 3.0, 1.0])
    {
        segment.best_segment_time_mut()[TimingMethod::GameTime] =
            Some(TimeSpan::from_seconds(best_segment_time));
    }
    run.regenerate_comparisons();
    run.segment_groups_mut()
        .push_lossy(1, 4, Some("Chapter A".into()), 6);

    let mut timer = Timer::new(run).unwrap();
    timer.set_current_timing_method(TimingMethod::GameTime);
    timer.start().unwrap();
    timer.initialize_game_time().unwrap();
    timer.pause_game_time().unwrap();
    for time in [0.0, 5.0, 18.0, 19.0] {
        timer.set_game_time(TimeSpan::from_seconds(time)).unwrap();
        timer.split().unwrap();
    }

    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        columns: vec![ColumnSettings {
            kind: ColumnKind::Time(TimeColumn {
                update_with: ColumnUpdateWith::Delta,
                comparison_override: Some(best_segments::NAME.into()),
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(state.splits[1].name, "Chapter A");
    assert_eq!(state.splits[1].columns[0].value, "+12.0");
    assert_eq!(
        state.splits[1].columns[0].semantic_color,
        SemanticColor::BehindLosingTime
    );
}

#[test]
fn blank_rows_reset_after_groups_collapse() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 4, Some("Chapter A".into()), 5);

    let mut timer = Timer::new(run).unwrap();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 7,
        fill_with_blank_space: true,
        subsplit_display_mode: SubsplitDisplayMode::CurrentGroupExpanded,
        ..english_settings()
    });
    let mut image_cache = ImageCache::new();
    let mut state = State::default();

    timer.start().unwrap();
    timer.split().unwrap();
    component.update_state(
        &mut state,
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.is_indented)
            .collect::<Vec<_>>(),
        [false, false, true, true, true, false, false]
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.section_index)
            .collect::<Vec<_>>(),
        [0, 1, 1, 1, 1, 2, 3]
    );

    timer.split().unwrap();
    timer.split().unwrap();
    timer.split().unwrap();
    component.update_state(
        &mut state,
        &mut image_cache,
        &timer.snapshot(),
        &Default::default(),
        Lang::English,
    );

    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.is_indented)
            .collect::<Vec<_>>(),
        [false; 7]
    );
    assert_eq!(
        state
            .splits
            .iter()
            .map(|s| s.section_index)
            .collect::<Vec<_>>(),
        [0, 1, 2, 3, 4, 5, 6]
    );
    for split in state.splits.iter().skip(3) {
        assert!(!split.is_indented);
    }
}
