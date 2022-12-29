pub mod shared;
pub mod entities;
pub mod repository;
pub mod request;
pub mod collections;
pub mod use_cases;
pub use shared::*;

#[cfg(test)]
mod tests {
    use crate::shared::Settings;

    use super::*;

    #[test]
    fn test_instance() {
        let result = Settings::instance().unwrap();
        assert_eq!(result.commondb().database(), "postgre_test", "DB wrong");
    }

    #[test]
    fn test_url_db() {
        let result = Settings::instance().unwrap();
        let url = result.commondb().database_url();
        assert_eq!(
            url, "postgresql://duonghb53:@localhost:5432/postgre_test?sslmode=disable",
            "DB wrong"
        );
    }
}
