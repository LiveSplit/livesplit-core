#[derive(Default, Clone)]
pub struct RunMetadata {
    run_id: String,
    platform_name: String,
    uses_emulator: bool,
    region_name: String,
    variables: Vec<(String, String)>,
}
