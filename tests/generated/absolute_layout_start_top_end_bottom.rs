#[test]
fn absolute_layout_start_top_end_bottom() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                position_type: stretch2::style::PositionType::Absolute,
                position: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(10f32),
                    end: stretch2::style::Dimension::Points(10f32),
                    top: stretch2::style::Dimension::Points(10f32),
                    bottom: stretch2::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(100f32),
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 80f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 80f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 10f32);
}
