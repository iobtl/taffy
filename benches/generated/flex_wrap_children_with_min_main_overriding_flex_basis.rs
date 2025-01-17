pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_basis: taffy::style::Dimension::Points(50f32),
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(50f32), ..Size::auto() },
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(55f32), ..Size::auto() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_basis: taffy::style::Dimension::Points(50f32),
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(50f32), ..Size::auto() },
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(55f32), ..Size::auto() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), ..Size::auto() },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
