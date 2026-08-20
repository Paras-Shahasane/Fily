use std::fs;

use fily_core::search::{search, SearchOptions};

fn temporary_test_directory() -> std::path::PathBuf {
    let unique_id = format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock is before Unix epoch")
            .as_nanos()
    );

    std::env::temp_dir().join(format!("fily-search-test-{unique_id}"))
}

#[test]
fn finds_matching_file() {
    let root = temporary_test_directory();

    fs::create_dir_all(&root)
        .expect("failed to create test directory");

    fs::write(root.join("report.txt"), b"report")
        .expect("failed to create report.txt");

    fs::write(root.join("notes.txt"), b"notes")
        .expect("failed to create notes.txt");

    let results = search(
        &root,
        "report",
        &SearchOptions::default(),
    )
    .expect("search failed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name(), "report.txt");

    fs::remove_dir_all(&root)
        .expect("failed to clean up");
}

#[test]
fn search_is_case_insensitive_by_default() {
    let root = temporary_test_directory();

    fs::create_dir_all(&root)
        .expect("failed to create test directory");

    fs::write(root.join("Report.txt"), b"report")
        .expect("failed to create file");

    let results = search(
        &root,
        "report",
        &SearchOptions::default(),
    )
    .expect("search failed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name(), "Report.txt");

    fs::remove_dir_all(&root)
        .expect("failed to clean up");
}

#[test]
fn case_sensitive_search_can_be_enabled() {
    let root = temporary_test_directory();

    fs::create_dir_all(&root)
        .expect("failed to create test directory");

    fs::write(root.join("Report.txt"), b"report")
        .expect("failed to create file");

    let options = SearchOptions {
        case_insensitive: false,
        ..SearchOptions::default()
    };

    let results = search(
        &root,
        "report",
        &options,
    )
    .expect("search failed");

    assert!(results.is_empty());

    fs::remove_dir_all(&root)
        .expect("failed to clean up");
}

#[test]
fn search_is_recursive_by_default() {
    let root = temporary_test_directory();
    let nested = root.join("Projects").join("Fily");

    fs::create_dir_all(&nested)
        .expect("failed to create nested directory");

    fs::write(
        nested.join("report.txt"),
        b"report",
    )
    .expect("failed to create report.txt");

    let results = search(
        &root,
        "report",
        &SearchOptions::default(),
    )
    .expect("search failed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name(), "report.txt");

    fs::remove_dir_all(&root)
        .expect("failed to clean up");
}

#[test]
fn recursive_search_can_be_disabled() {
    let root = temporary_test_directory();
    let nested = root.join("nested");

    fs::create_dir_all(&nested)
        .expect("failed to create nested directory");

    fs::write(
        nested.join("report.txt"),
        b"report",
    )
    .expect("failed to create report.txt");

    let options = SearchOptions {
        recursive: false,
        ..SearchOptions::default()
    };

    let results = search(
        &root,
        "report",
        &options,
    )
    .expect("search failed");

    assert!(results.is_empty());

    fs::remove_dir_all(&root)
        .expect("failed to clean up");
}

#[test]
fn hidden_entries_are_excluded_by_default() {
    let root = temporary_test_directory();

    fs::create_dir_all(&root)
        .expect("failed to create test directory");

    fs::write(
        root.join(".hidden-report"),
        b"hidden",
    )
    .expect("failed to create hidden file");

    let results = search(
        &root,
        "report",
        &SearchOptions::default(),
    )
    .expect("search failed");

    assert!(results.is_empty());

    fs::remove_dir_all(&root)
        .expect("failed to clean up");
}

#[test]
fn hidden_entries_can_be_included() {
    let root = temporary_test_directory();

    fs::create_dir_all(&root)
        .expect("failed to create test directory");

    fs::write(
        root.join(".hidden-report"),
        b"hidden",
    )
    .expect("failed to create hidden file");

    let options = SearchOptions {
        include_hidden: true,
        ..SearchOptions::default()
    };

    let results = search(
        &root,
        "report",
        &options,
    )
    .expect("search failed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name(), ".hidden-report");

    fs::remove_dir_all(&root)
        .expect("failed to clean up");
}