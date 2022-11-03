#[test]
fn test_tensor_display() {
    let tensor = super::Tensor::<f32, 4, 4>::from([
        [1.0f32, 0.0f32, 0.0f32, 0.0f32],
        [0.0f32, 1.0f32, 0.0f32, 0.0f32],
        [0.0f32, 0.0f32, 1.0f32, 0.0f32],
        [0.0f32, 0.0f32, 0.0f32, 1.0f32],
    ]);

    assert!(format!("{:.2}", tensor) == "[1.00, 0.00, 0.00, 0.00 | 0.00, 1.00, 0.00, 0.00 | 0.00, 0.00, 1.00, 0.00 | 0.00, 0.00, 0.00, 1.00]",
        "Erroneous formatting of 4x4 Tensor display");
    
    let tensor = super::Tensor::<f32, 1, 4>::from([
        [1.0f32, 0.0f32, 0.1f32, 0.0f32],
    ]);

    assert!(format!("{:.3}", tensor) == "[1.000, 0.000, 0.100, 0.000]",
        "Erroneous formatting of 1x4 Tensor display");
    
    let tensor = super::Tensor::<f32, 4, 1>::from([
        [1.0f32], [2.0f32], [0.1f32], [0.0f32],
    ]);

    assert!(format!("{:.3}", tensor) == "[1.000 | 2.000 | 0.100 | 0.000]",
        "Erroneous formatting of 4x1 Tensor display");
}