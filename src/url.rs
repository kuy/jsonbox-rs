const ENDPOINT: &str = "https://jsonbox.io";

pub fn of_box(box_id: &str) -> String {
    format!("{}/{}", ENDPOINT, box_id)
}

pub fn of_record(box_id: &str, record_id: &str) -> String {
    format!("{}/{}/{}", ENDPOINT, box_id, record_id)
}
