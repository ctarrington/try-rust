use data_cache::api::cache::{Cache, CacheError};
use uuid::Uuid;

fn create_flavors() -> Vec<String> {
    vec![
        "vanilla".to_string(),
        "chocolate".to_string(),
        "strawberry".to_string(),
    ]
}

fn create_cache() -> Cache {
    let mut cache = Cache::new();
    cache.add_string_column("name", "Name", "unknown").unwrap();
    cache
        .add_boolean_column("verified", "Verified", "false")
        .unwrap();
    cache.add_f64_column("age", "Age", "0").unwrap();
    cache
        .add_time_date_column("start_time", "Start Time", "%Y-%m-%d %H:%M:%S", "")
        .unwrap();
    cache
        .add_enumerated_column("flavor", "Flavor", "vanilla", create_flavors())
        .unwrap();
    cache
}

#[test]
fn test_simple() {
    let mut cache = create_cache();

    let fred_guid = cache
        .create_row("fred,true, 1, 2019-01-01 00:00:00,chocolate")
        .unwrap();
    let empty_guid = cache.create_row(",,,,").unwrap();

    assert_eq!(
        cache.csv_for_guid(&fred_guid).unwrap(),
        "fred,true,1,2019-01-01 00:00:00,chocolate"
    );

    assert_eq!(
        cache.csv_for_guid(&empty_guid).unwrap(),
        "unknown,false,0,,vanilla"
    );
}

#[test]
fn test_invalid_rows() {
    let mut cache = create_cache();

    assert!(cache
        .create_row("wilma,false, 2020-01-01 00:00:00,1")
        .is_err());
    assert!(cache.create_row("").is_err());
    assert!(cache
        .create_row("wilma,false, 2020-01-01 00:00:00,")
        .is_err());
}

#[test]
fn test_empty() {
    let mut cache = create_cache();

    let guid = cache.create_row(",,,,").unwrap();
    assert_eq!(
        cache.csv_for_guid(&guid).unwrap(),
        "unknown,false,0,,vanilla"
    );
}

#[test]
fn test_valid_after_invalid() {
    let mut cache = create_cache();

    assert!(cache.create_row("wilma,false,1").is_err());
    let fred_guid = cache
        .create_row("fred,true, 1, 2019-01-01 00:00:00,strawberry")
        .unwrap();
    assert_eq!(
        cache.csv_for_guid(&fred_guid).unwrap(),
        "fred,true,1,2019-01-01 00:00:00,strawberry"
    );
}

#[test]
fn test_default_gets_added() {
    let flavors = create_flavors(); // does not include "fudge ripple"
    let mut cache = Cache::new();
    cache
        .add_enumerated_column("flavor", "Flavor", "fudge ripple", flavors)
        .unwrap();
    let guid = cache.create_row("").unwrap();
    assert_eq!(cache.csv_for_guid(&guid).unwrap(), "fudge ripple");
}

#[test]
fn test_empty_default() {
    let flavors = create_flavors();
    let mut cache = Cache::new();
    cache
        .add_enumerated_column("flavor", "Flavor", "", flavors)
        .unwrap();
    let guid = cache.create_row("").unwrap();
    assert_eq!(cache.csv_for_guid(&guid).unwrap(), "");
}

#[test]
fn test_add_column_to_existing_cache() {
    let mut cache = Cache::new();
    cache.add_string_column("name", "Name", "unknown").unwrap();
    let fred_guid = cache.create_row("fred").unwrap();
    let wilma_guid = cache.create_row("wilma").unwrap();
    assert_eq!(cache.csv_for_guid(&fred_guid).unwrap(), "fred");
    assert_eq!(cache.csv_for_guid(&wilma_guid).unwrap(), "wilma");

    cache.add_f64_column("height", "Height", "0").unwrap();
    let barney_guid = cache.create_row("barney,60").unwrap();
    assert_eq!(cache.csv_for_guid(&fred_guid).unwrap(), "fred,0");
    assert_eq!(cache.csv_for_guid(&wilma_guid).unwrap(), "wilma,0");
    assert_eq!(cache.csv_for_guid(&barney_guid).unwrap(), "barney,60");

    cache
        .add_time_date_column(
            "start_time",
            "Start Time",
            "%Y-%m-%d %H:%M:%S",
            "1999-01-01 00:00:00",
        )
        .unwrap();
    assert_eq!(
        cache.csv_for_guid(&fred_guid).unwrap(),
        "fred,0,1999-01-01 00:00:00"
    );
    assert_eq!(
        cache.csv_for_guid(&wilma_guid).unwrap(),
        "wilma,0,1999-01-01 00:00:00"
    );
    assert_eq!(
        cache.csv_for_guid(&barney_guid).unwrap(),
        "barney,60,1999-01-01 00:00:00"
    );

    let pebbles_guid = cache.create_row("pebbles,10,2020-01-01 00:00:00").unwrap();
    assert_eq!(
        cache.csv_for_guid(&pebbles_guid).unwrap(),
        "pebbles,10,2020-01-01 00:00:00"
    );

    cache
        .add_boolean_column("verified", "Verified", "false")
        .unwrap();
    assert_eq!(
        cache.csv_for_guid(&fred_guid).unwrap(),
        "fred,0,1999-01-01 00:00:00,false"
    );

    cache
        .add_enumerated_column("flavor", "Flavor", "vanilla", create_flavors())
        .unwrap();
    assert_eq!(
        cache.csv_for_guid(&fred_guid).unwrap(),
        "fred,0,1999-01-01 00:00:00,false,vanilla"
    );

    cache
        .add_string_column("address", "Address", "unknown")
        .unwrap();
    assert_eq!(
        cache.csv_for_guid(&fred_guid).unwrap(),
        "fred,0,1999-01-01 00:00:00,false,vanilla,unknown"
    );
}

#[test]
fn test_default_cache() {
    let mut cache: Cache = Default::default();
    cache.add_string_column("name", "Name", "unknown").unwrap();
    let fred_guid = cache.create_row("fred").unwrap();
    assert_eq!(cache.csv_for_guid(&fred_guid).unwrap(), "fred");
}

#[test]
fn test_update_row() {
    let mut cache = create_cache();

    let guid = cache
        .create_row("fred,true, 1, 2019-01-01 00:00:00,chocolate")
        .unwrap();
    assert_eq!(
        cache.csv_for_guid(&guid).unwrap(),
        "fred,true,1,2019-01-01 00:00:00,chocolate"
    );

    cache
        .update_row(&guid, "wilma,false, 2, 2020-01-01 00:00:00,strawberry")
        .unwrap();
    assert_eq!(
        cache.csv_for_guid(&guid).unwrap(),
        "wilma,false,2,2020-01-01 00:00:00,strawberry"
    );
}

#[test]
fn test_update_row_invalid() {
    let mut cache = create_cache();

    let guid = cache
        .create_row("fred,true, 1, 2019-01-01 00:00:00,chocolate")
        .unwrap();
    assert_eq!(
        cache.csv_for_guid(&guid).unwrap(),
        "fred,true,1,2019-01-01 00:00:00,chocolate"
    );

    assert!(cache
        .update_row(
            &guid,
            "wilma,false, invalid number, 2020-01-01 00:00:00,strawberry"
        )
        .is_err());
    assert_eq!(
        cache.csv_for_guid(&guid).unwrap(),
        "fred,true,1,2019-01-01 00:00:00,chocolate"
    );
}

#[test]
fn test_missing_guid() {
    let mut cache = create_cache();

    let guid = cache
        .create_row("fred,true, 1, 2019-01-01 00:00:00,chocolate")
        .unwrap();
    assert_eq!(
        cache.csv_for_guid(&guid).unwrap(),
        "fred,true,1,2019-01-01 00:00:00,chocolate"
    );

    let other_guid = Uuid::new_v4();
    assert_eq!(
        cache.update_row(
            &other_guid,
            "wilma,false, 4, 2020-01-01 00:00:00,strawberry"
        ),
        Err(CacheError::GuidNotFound(other_guid))
    );
    assert_eq!(
        cache.csv_for_guid(&guid).unwrap(),
        "fred,true,1,2019-01-01 00:00:00,chocolate"
    );
}

#[test]
fn test_duplicate_column() {
    let mut cache = create_cache();

    assert_eq!(
        cache.add_string_column("name", "Name", "unknown"),
        Err(CacheError::DuplicateColumn("name".to_string()))
    );

    assert_eq!(
        cache.add_boolean_column("verified", "Verified", "false"),
        Err(CacheError::DuplicateColumn("verified".to_string()))
    );

    assert_eq!(
        cache.add_f64_column("age", "Age", "0"),
        Err(CacheError::DuplicateColumn("age".to_string()))
    );

    assert_eq!(
        cache.add_time_date_column("start_time", "Start Time", "%Y-%m-%d %H:%M:%S", ""),
        Err(CacheError::DuplicateColumn("start_time".to_string()))
    );

    assert_eq!(
        cache.add_enumerated_column("flavor", "Flavor", "vanilla", create_flavors()),
        Err(CacheError::DuplicateColumn("flavor".to_string()))
    );
}

#[test]
fn test_duplicate_column_format() {
    let mut cache = create_cache();
    let result = cache.add_time_date_column("start_time", "Start Time", "%Y-%m-%d %H:%M:%S", "");
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.to_string(), "Duplicate column: start_time");
    }
}

#[test]
fn test_missing_guid_format() {
    let mut cache = create_cache();
    let other_guid = Uuid::new_v4();
    let result = cache.update_row(
        &other_guid,
        "wilma,false, 4, 2020-01-01 00:00:00,strawberry",
    );
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.to_string(), format!("Guid not found: {}", other_guid));
    }
}

#[test]
fn test_invalid_row_format() {
    let mut cache = create_cache();
    let result = cache.create_row("wilma,invalid boolean, 5, 2020-01-01 00:00:00,strawberry");
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(
            error.to_string(),
            "ParseError: provided string was not `true` or `false`"
        );
    }
}
