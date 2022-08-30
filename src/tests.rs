use crate::__private::utils::get_database_from_url;

#[test]
fn get_database_test() {
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379").unwrap()),
        None
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/0").unwrap()),
        Some(0)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/0/").unwrap()),
        Some(0)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/0/foo").unwrap()),
        Some(0)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/1/foo/").unwrap()),
        Some(1)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/0/foo/bar").unwrap()),
        Some(0)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/1/foo/bar/").unwrap()),
        Some(1)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/asd/foo/bar/").unwrap()),
        None
    );
}
