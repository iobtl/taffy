pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(60f32),
                    height: taffy::style::Dimension::Points(40f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: taffy::style::AlignItems::Center,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(110f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
