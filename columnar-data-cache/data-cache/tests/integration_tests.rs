use data_cache::api::cache::Cache;

#[test]
fn test_new_cache() {
    let mut cache = Cache::new();
    cache.add_string_column("name", "Name", "unknown");
    cache.add_boolean_column("verified", "Verified", "false");
    cache.add_f64_column("age", "Age", "0");
    cache.add_time_date_column("start_time", "Start Time", "%Y-%m-%d %H:%M:%S");

    let flavors = vec![
        "vanilla".to_string(),
        "chocolate".to_string(),
        "strawberry".to_string(),
    ];
    cache.add_enumerated_column("flavor", "Flavor", "vanilla", flavors);

    cache
        .add_row("ted,false, 4, 2020-02-02 05:06:07,chocolate")
        .unwrap();

    assert!(cache
        .add_row("wilma,false, 2020-01-01 00:00:00,fudge")
        .is_err());

    assert_eq!(
        cache.row_as_csv(0).unwrap(),
        "ted,false,4,2020-02-02 05:06:07,chocolate"
    );
}

#[test]
fn test_default_cache() {
    let mut cache: Cache = Default::default();
    cache.add_string_column("name", "Name", "unknown");
    cache.add_row("fred").unwrap();
    assert_eq!(cache.row_as_csv(0).unwrap(), "fred");
}
