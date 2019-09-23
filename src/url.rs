const ENDPOINT: &str = "https://jsonbox.io";

pub fn of_box(box_id: &str) -> String {
    format!("{}/{}", ENDPOINT, box_id)
}

pub fn of_record(box_id: &str, record_id: &str) -> String {
    format!("{}/{}/{}", ENDPOINT, box_id, record_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_of_box() {
        assert_eq!(
            of_box("01234567890123456789"),
            "https://jsonbox.io/01234567890123456789"
        );
    }

    #[test]
    fn test_of_record() {
        assert_eq!(
            of_record("01234567890123456789", "5d876d852a780700177c0557"),
            "https://jsonbox.io/01234567890123456789/5d876d852a780700177c0557"
        );
    }
}
