use std::fs;

use fily_core::filesystem::{EntryType, FileEntry};

#[test]
fn file_entry_reports_file_information() {
    let temp_dir = std::env::temp_dir().join(format!(
        "fily-entry-test-{}",
        std::process::id()
    ));

    fs::create_dir_all(&temp_dir)
        .expect("failed to create temporary directory");

    let file_path = temp_dir.join("example.txt");

    fs::write(&file_path, b"hello fily")
        .expect("failed to create test file");

    let entry =
        FileEntry::from_path(file_path.clone())
            .expect("failed to create FileEntry");

    assert_eq!(entry.name(), "example.txt");
    assert_eq!(entry.path(), &file_path);
    assert_eq!(entry.entry_type(), EntryType::File);
    assert!(entry.is_file());
    assert!(!entry.is_directory());
    assert!(!entry.is_symlink());
    assert_eq!(entry.size(), 10);
    assert!(!entry.is_hidden());

    fs::remove_dir_all(&temp_dir)
        .expect("failed to clean up temporary directory");
}

#[test]
fn file_entry_reports_directory_information() {
    let temp_dir = std::env::temp_dir().join(format!(
        "fily-entry-directory-test-{}",
        std::process::id()
    ));

    fs::create_dir_all(&temp_dir)
        .expect("failed to create temporary directory");

    let entry =
        FileEntry::from_path(temp_dir.clone())
            .expect("failed to create FileEntry");

    assert_eq!(entry.name(), temp_dir.file_name().unwrap().to_string_lossy());
    assert_eq!(entry.path(), &temp_dir);
    assert_eq!(entry.entry_type(), EntryType::Directory);
    assert!(entry.is_directory());
    assert!(!entry.is_file());
    assert!(!entry.is_symlink());
    assert_eq!(entry.size(), 0);

    fs::remove_dir_all(&temp_dir)
        .expect("failed to clean up temporary directory");
}

#[test]
fn hidden_file_is_detected() {
    let temp_dir = std::env::temp_dir().join(format!(
        "fily-entry-hidden-test-{}",
        std::process::id()
    ));

    fs::create_dir_all(&temp_dir)
        .expect("failed to create temporary directory");

    let file_path = temp_dir.join(".hidden");

    fs::write(&file_path, b"hidden")
        .expect("failed to create hidden file");

    let entry =
        FileEntry::from_path(file_path.clone())
            .expect("failed to create FileEntry");

    assert_eq!(entry.name(), ".hidden");
    assert!(entry.is_hidden());
    assert!(entry.is_file());

    fs::remove_dir_all(&temp_dir)
        .expect("failed to clean up temporary directory");
}