pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                margin: taffy::geometry::Rect {
                    right: taffy::style::LengthPercentageAuto::Points(10f32),
                    ..Rect::zero()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy.new_with_children(taffy::style::Style { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
