use crate::Tensor;
use crate::accelerator::Factory;
use crate::tensor;

#[test]
fn test_open_cl() {
    let mut factory = Factory::new();
    let devices = factory.get_devices().unwrap();

    println!("Device: {}", devices.first().unwrap().name().unwrap());
    println!("Device Type: {}", devices.first().unwrap().device_type().unwrap());
    println!("Device Vendor: {}", devices.first().unwrap().vendor().unwrap());
}

#[test]
fn test_tensor() {
    let matrix : Tensor<f64, 4, 4> = tensor!(
        1.0f64, 0.0f64, 0.0f64, 0.0f64 |
        0.0f64, 1.0f64, 0.0f64, 0.0f64 |
        0.0f64, 2.0f64, 1.0f64, 0.0f64 |
        0.0f64, 0.0f64, 0.0f64, 1.0f64);

    assert!(matrix[(2, 1)] == 2.0,
        "Erroneous Tensor index operation");

    assert!(format!("{:.2}", matrix) == "[1.00, 0.00, 0.00, 0.00 | 0.00, 1.00, 0.00, 0.00 | 0.00, 2.00, 1.00, 0.00 | 0.00, 0.00, 0.00, 1.00]",
        "Erroneous formatting of 4x4 Tensor display");
    
    let rvector = tensor![1.0f64, 0.0f64, 0.1f64, 0.0f64];

    assert!(format!("{:.3}", rvector) == "[1.000, 0.000, 0.100, 0.000]",
        "Erroneous formatting of 1x4 Tensor display");
    
    let cvector = tensor![1.0f64 | 2.0f64 | 0.1f64 | 0.0f64];

    assert!(format!("{:.3}", cvector) == "[1.000 | 2.000 | 0.100 | 0.000]",
        "Erroneous formatting of 4x1 Tensor display");
}