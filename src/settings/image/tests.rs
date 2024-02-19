use super::Image;

#[test]
fn serializes_to_json_as_data_url() {
    let json = serde_json::to_string(&Image::new([1, 2, 3].into(), Image::ICON)).unwrap();
    assert_eq!(r#""AQID""#, json);
}
