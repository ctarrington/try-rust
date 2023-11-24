use data_cache::api::cache::Cache;

fn create_flavors() -> Vec<String> {
    vec![
        "vanilla".to_string(),
        "chocolate".to_string(),
        "strawberry".to_string(),
    ]
}

fn create_cache() -> Cache {
    let mut cache = Cache::new();
    cache.add_string_column("name", "Name", "unknown");
    cache.add_boolean_column("verified", "Verified", "false");
    cache.add_f64_column("age", "Age", "0");
    cache.add_time_date_column("start_time", "Start Time", "%Y-%m-%d %H:%M:%S", "");
    cache.add_enumerated_column("flavor", "Flavor", "vanilla", create_flavors());
    cache
}

#[test]
fn test_simple() {
    let mut cache = create_cache();

    assert!(cache.row_as_csv(0).is_err());
    cache
        .add_row("fred,true, 1, 2019-01-01 00:00:00,chocolate")
        .unwrap();
    cache.add_row(",,,,").unwrap();

    assert_eq!(
        cache.row_as_csv(0).unwrap(),
        "fred,true,1,2019-01-01 00:00:00,chocolate"
    );

    assert_eq!(cache.row_as_csv(1).unwrap(), "unknown,false,0,,vanilla");
}

#[test]
fn test_invalid_rows() {
    let mut cache = create_cache();

    assert!(cache.add_row("wilma,false, 2020-01-01 00:00:00,1").is_err());
    assert!(cache.add_row("").is_err());
    assert!(cache.add_row("wilma,false, 2020-01-01 00:00:00,").is_err());
}

#[test]
fn test_empty() {
    let mut cache = create_cache();

    cache.add_row(",,,,").unwrap();
    assert_eq!(cache.row_as_csv(0).unwrap(), "unknown,false,0,,vanilla");
}

#[test]
fn test_valid_after_invalid() {
    let mut cache = create_cache();

    assert!(cache.add_row("wilma,false,1").is_err());
    cache
        .add_row("fred,true, 1, 2019-01-01 00:00:00,strawberry")
        .unwrap();
    assert_eq!(
        cache.row_as_csv(0).unwrap(),
        "fred,true,1,2019-01-01 00:00:00,strawberry"
    );
}

#[test]
fn test_default_gets_added() {
    let flavors = create_flavors(); // does not include "fudge ripple"
    let mut cache = Cache::new();
    cache.add_enumerated_column("flavor", "Flavor", "fudge ripple", flavors);
    cache.add_row("").unwrap();
    assert_eq!(cache.row_as_csv(0).unwrap(), "fudge ripple");
}

#[test]
fn test_empty_default() {
    let flavors = create_flavors();
    let mut cache = Cache::new();
    cache.add_enumerated_column("flavor", "Flavor", "", flavors);
    cache.add_row("").unwrap();
    assert_eq!(cache.row_as_csv(0).unwrap(), "");
}

#[test]
fn test_add_column_to_existing_cache() {
    let mut cache = Cache::new();
    cache.add_string_column("name", "Name", "unknown");
    cache.add_row("fred").unwrap();
    cache.add_row("wilma").unwrap();
    assert_eq!(cache.row_as_csv(0).unwrap(), "fred");
    assert_eq!(cache.row_as_csv(1).unwrap(), "wilma");

    cache.add_f64_column("height", "Height", "0");
    cache.add_row("barney,60").unwrap();
    assert_eq!(cache.row_as_csv(0).unwrap(), "fred,0");
    assert_eq!(cache.row_as_csv(1).unwrap(), "wilma,0");
    assert_eq!(cache.row_as_csv(2).unwrap(), "barney,60");

    cache.add_time_date_column("start_time", "Start Time", "%Y-%m-%d %H:%M:%S", "");
    assert_eq!(cache.row_as_csv(0).unwrap(), "fred,0,");
    cache.add_row("pebbles,10,2020-01-01 00:00:00").unwrap();
    assert_eq!(
        cache.row_as_csv(3).unwrap(),
        "pebbles,10,2020-01-01 00:00:00"
    );

    cache.add_boolean_column("verified", "Verified", "false");
    assert_eq!(cache.row_as_csv(0).unwrap(), "fred,0,,false");

    cache.add_enumerated_column("flavor", "Flavor", "vanilla", create_flavors());
    assert_eq!(cache.row_as_csv(0).unwrap(), "fred,0,,false,vanilla");

    cache.add_string_column("address", "Address", "unknown");
    assert_eq!(
        cache.row_as_csv(0).unwrap(),
        "fred,0,,false,vanilla,unknown"
    );
}

#[test]
fn test_default_cache() {
    let mut cache: Cache = Default::default();
    cache.add_string_column("name", "Name", "unknown");
    cache.add_row("fred").unwrap();
    assert_eq!(cache.row_as_csv(0).unwrap(), "fred");
}
