use crate::{
    Run, Segment,
    run::{SegmentGroup, SegmentGroups},
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
    assert_eq!(views[1].major_index(), 3);
    assert_eq!(views[2].name_or_default(), "Outro");
}

#[test]
fn segment_group_icons_default_to_major_segment_icon() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End"] {
        run.push_segment(Segment::new(name));
    }

    let major_icon = Image::new([1, 2, 3].as_slice().into(), Image::ICON);
    run.segment_mut(2).set_icon(major_icon.clone());
    run.segment_groups_mut()
        .push_lossy(1, 3, Some("Chapter A".into()), 3);

    let views = run.segment_groups_iter().collect::<Vec<_>>();
    assert_eq!(views[1].icon(), None);
    assert_eq!(views[1].icon_or_default().id(), major_icon.id());

    let group_icon = Image::new([4, 5, 6].as_slice().into(), Image::ICON);
    run.segment_groups_mut().groups_mut()[0].set_icon(group_icon.clone());

    let views = run.segment_groups_iter().collect::<Vec<_>>();
    assert_eq!(views[1].icon().unwrap().id(), group_icon.id());
    assert_eq!(views[1].icon_or_default().id(), group_icon.id());
}
