use crate::{
    Run, Segment,
    run::{SegmentGroup, SegmentGroups},
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

    assert_eq!(groups.groups().len(), 2);
    assert_eq!(
        (groups.groups()[0].start(), groups.groups()[0].end()),
        (1, 4)
    );
    assert_eq!(
        (groups.groups()[1].start(), groups.groups()[1].end()),
        (4, 6)
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
