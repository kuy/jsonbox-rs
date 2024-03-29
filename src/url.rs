pub const BASE_URL: &'static str = "https://jsonbox.io";

pub fn of_box(base_url: &str, box_id: &str) -> String {
    format!("{}/{}", base_url, box_id)
}

pub fn of_record(base_url: &str, box_id: &str, record_id: &str) -> String {
    format!("{}/{}/{}", base_url, box_id, record_id)
}

pub fn of_query(base_url: &str, box_id: &str, query: &str) -> String {
    format!("{}/{}?{}", base_url, box_id, query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_of_box() {
        assert_eq!(
            of_box("https://jsonbox.io", "01234567890123456789"),
            "https://jsonbox.io/01234567890123456789"
        );
    }

    #[test]
    fn test_of_record() {
        assert_eq!(
            of_record(
                "https://jsonbox.io",
                "01234567890123456789",
                "5d876d852a780700177c0557"
            ),
            "https://jsonbox.io/01234567890123456789/5d876d852a780700177c0557"
        );
    }

    #[test]
    fn test_of_query() {
        assert_eq!(
            of_query(
                "https://jsonbox.io",
                "01234567890123456789",
                "sort=-createdOn&limit=20&skip=0"
            ),
            "https://jsonbox.io/01234567890123456789?sort=-createdOn&limit=20&skip=0"
        );
    }
}
