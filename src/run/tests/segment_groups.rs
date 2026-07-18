use crate::{
    Run, Segment,
    run::{SegmentGroup, SegmentGroupError, SegmentGroups},
    settings::Image,
};

#[test]
fn segment_groups_repair_invalid_ranges() {
    let groups = SegmentGroups::from_vec_lossy(
        vec![
            SegmentGroup::new_unchecked(4, 6, Some("B".into())),
            SegmentGroup::new_unchecked(1, 4, Some("A".into())),
            SegmentGroup::new_unchecked(3, 3, Some("Dropped".into())),
            SegmentGroup::new_unchecked(5, 10, Some("Clamped".into())),
        ],
        7,
    );

    assert_eq!(groups.groups().len(), 3);
    assert_eq!(
        (groups.groups()[0].start(), groups.groups()[0].end()),
        (1, 4)
    );
    assert_eq!(
        (groups.groups()[1].start(), groups.groups()[1].end()),
        (4, 6)
    );
    assert_eq!(
        (groups.groups()[2].start(), groups.groups()[2].end()),
        (6, 7)
    );
}

#[test]
fn segment_group_range_updates_are_exact_and_atomic() {
    let mut groups = SegmentGroups::from_vec_lossy(
        vec![
            SegmentGroup::new(0, 2, Some("A".into())).unwrap(),
            SegmentGroup::new(2, 5, Some("B".into())).unwrap(),
        ],
        5,
    );

    // Swapping two adjacent groups requires changing both ranges at once. The
    // group metadata follows its identity, while the collection restores its
    // range ordering only after validating the complete update.
    groups.set_ranges([(0, 3..5), (1, 0..3)], 5).unwrap();
    assert_eq!(groups.groups()[0].name(), Some("B"));
    assert_eq!(groups.groups()[1].name(), Some("A"));

    let unchanged = groups.clone();
    assert_eq!(
        groups.set_range(0, 0..4, 5),
        Err(SegmentGroupError::OverlappingRanges)
    );
    assert_eq!(
        groups.set_range(0, 1..1, 5),
        Err(SegmentGroupError::EmptyRange)
    );
    assert_eq!(
        groups.set_range(0, 0..6, 5),
        Err(SegmentGroupError::RangeOutOfBounds)
    );
    assert_eq!(
        groups.set_range(2, 0..1, 5),
        Err(SegmentGroupError::InvalidIndex)
    );
    assert_eq!(groups, unchanged);
}

#[test]
fn empty_segment_group_range_is_reported() {
    assert_eq!(
        SegmentGroup::new(2, 2, None),
        Err(SegmentGroupError::EmptyRange)
    );
}

#[test]
fn segment_groups_iterates_grouped_and_ungrouped_segments() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    run.segment_groups_mut()
        .push_lossy(1, 4, Some("Chapter A".into()), 5);

    let views = run.segment_groups_iter().collect::<Vec<_>>();

    assert_eq!(views.len(), 3);
    assert_eq!(views[0].name_or_default(), "Intro");
    assert_eq!(views[1].name_or_default(), "Chapter A");
    assert_eq!(views[1].last_segment_index(), 3);
    assert_eq!(views[2].name_or_default(), "Outro");
}

#[test]
fn segment_group_icons_default_to_last_segment_icon() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End"] {
        run.push_segment(Segment::new(name));
    }

    let last_segment_icon = Image::new([1, 2, 3].as_slice().into(), Image::ICON);
    run.segment_mut(2).set_icon(last_segment_icon.clone());
    run.segment_groups_mut()
        .push_lossy(1, 3, Some("Chapter A".into()), 3);

    let views = run.segment_groups_iter().collect::<Vec<_>>();
    assert_eq!(views[1].icon(), None);
    assert_eq!(views[1].icon_or_default().id(), last_segment_icon.id());

    let group_icon = Image::new([4, 5, 6].as_slice().into(), Image::ICON);
    run.segment_groups_mut()
        .set_icon(0, group_icon.clone())
        .unwrap();

    let views = run.segment_groups_iter().collect::<Vec<_>>();
    assert_eq!(views[1].icon().unwrap().id(), group_icon.id());
    assert_eq!(views[1].icon_or_default().id(), group_icon.id());
}

#[test]
fn structural_segment_mutations_keep_group_ranges_valid() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }
    let segment_count = run.len();
    run.segment_groups_mut()
        .push_lossy(1, 3, Some("Chapter A".into()), segment_count);

    // Inserting directly before a group must shift the whole group rather than
    // making the new segment its first member.
    run.insert_segment(0, Segment::new("Prologue"));
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
        ),
        (2, 4),
    );

    // Removing that ungrouped segment restores the original range. Iterating
    // afterwards exercises the invariant that every group range is in bounds.
    run.remove_segment(0);
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
        ),
        (1, 3),
    );
    assert_eq!(
        run.segment_groups_iter()
            .map(|view| view.name_or_default().to_owned())
            .collect::<Vec<_>>(),
        ["Intro", "Chapter A", "Outro"],
    );
}
