#[test]
fn border_center_child() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(10f32),
                    height: taffy::style::Dimension::Points(10f32),
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
                justify_content: taffy::style::JustifyContent::Center,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Size::auto()
                },
                border: taffy::geometry::Rect {
                    top: taffy::style::LengthPercentage::Points(10f32),
                    bottom: taffy::style::LengthPercentage::Points(20f32),
                    ..Rect::zero()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 45f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 40f32);
}
