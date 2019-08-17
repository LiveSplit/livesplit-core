use super::ImageData;

#[test]
fn serializes_to_json_as_data_url() {
    let json = serde_json::to_string(&ImageData(vec![1, 2, 3].into_boxed_slice())).unwrap();
    assert_eq!(r#""data:;base64,AQID""#, json);
}
