#[test]
fn justify_content_row_min_width_and_margin() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(20f32),
                    height: stretch2::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                margin: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(10f32),
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
                justify_content: stretch2::style::JustifyContent::Center,
                min_size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 20f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 20f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 20f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 20f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
}
