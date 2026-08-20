use fily_core::filesystem::operations;
use fily_core::filesystem::path::FilyPath;
use fily_core::filesystem::policy::CollisionPolicy;

use std::time::{SystemTime, UNIX_EPOCH};

fn temporary_test_directory() -> FilyPath {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before UNIX epoch")
        .as_nanos();

    let path = std::env::temp_dir()
        .join(format!("fily-core-test-{timestamp}"));

    FilyPath::new(path)
}

#[test]
fn create_file_and_verify_it_exists() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let file = directory.join("test.txt");

    operations::create_file(&file)
        .expect("failed to create test file");

    assert!(file.exists());
    assert!(file.is_file());

    operations::delete_file(&file)
        .expect("failed to delete test file");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn create_directory_and_list_contents() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let file = directory.join("hello.txt");
    let subdirectory = directory.join("subdirectory");

    operations::create_file(&file)
        .expect("failed to create test file");

    operations::create_directory(&subdirectory)
        .expect("failed to create test subdirectory");

    let listing = operations::list_directory(&directory)
        .expect("failed to list directory");

    assert_eq!(listing.files.len(), 1);
    assert_eq!(listing.directories.len(), 1);

    assert_eq!(listing.files[0].name(), "hello.txt");
    assert_eq!(listing.directories[0].name(), "subdirectory");

    operations::delete_file(&file)
        .expect("failed to delete test file");

    operations::delete_directory(&subdirectory)
        .expect("failed to delete test subdirectory");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn rename_file() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let original = directory.join("original.txt");
    let renamed = directory.join("renamed.txt");

    operations::create_file(&original)
        .expect("failed to create test file");

    operations::rename(
    &original,
    &renamed,
    CollisionPolicy::Fail,
)
.expect("failed to rename file");

    assert!(!original.exists());
    assert!(renamed.exists());

    operations::delete_file(&renamed)
        .expect("failed to delete renamed file");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn copy_file() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let source = directory.join("source.txt");
    let destination = directory.join("copy.txt");

    operations::create_file(&source)
        .expect("failed to create source file");

    operations::copy_file(
    &source,
    &destination,
    CollisionPolicy::Fail,
)
.expect("failed to copy file");

    assert!(source.exists());
    assert!(destination.exists());

    operations::delete_file(&source)
        .expect("failed to delete source file");

    operations::delete_file(&destination)
        .expect("failed to delete copied file");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn move_file() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let source = directory.join("source.txt");
    let destination = directory.join("moved.txt");

    operations::create_file(&source)
        .expect("failed to create source file");

    operations::move_path(
    &source,
    &destination,
    CollisionPolicy::Fail,
)
.expect("failed to move file");

    assert!(!source.exists());
    assert!(destination.exists());

    operations::delete_file(&destination)
        .expect("failed to delete moved file");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn metadata_is_reported() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let file = directory.join("metadata.txt");

    operations::create_file(&file)
        .expect("failed to create test file");

    let metadata = operations::metadata(&file)
        .expect("failed to read metadata");

    assert!(metadata.is_file());
    assert!(!metadata.is_directory());
    assert!(metadata.created.is_some() || metadata.modified.is_some());

    operations::delete_file(&file)
        .expect("failed to delete test file");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn copy_file_fails_when_destination_exists() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let source = directory.join("source.txt");
    let destination = directory.join("destination.txt");

    operations::create_file(&source)
        .expect("failed to create source file");

    operations::create_file(&destination)
        .expect("failed to create destination file");

    let result = operations::copy_file(
        &source,
        &destination,
        CollisionPolicy::Fail,
    );

    assert!(result.is_err());

    operations::delete_file(&source)
        .expect("failed to delete source file");

    operations::delete_file(&destination)
        .expect("failed to delete destination file");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn copy_file_skips_existing_destination() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let source = directory.join("source.txt");
    let destination = directory.join("destination.txt");

    operations::create_file(&source)
        .expect("failed to create source file");

    operations::create_file(&destination)
        .expect("failed to create destination file");

    operations::copy_file(
        &source,
        &destination,
        CollisionPolicy::Skip,
    )
    .expect("skip policy should succeed");

    assert!(source.exists());
    assert!(destination.exists());

    operations::delete_file(&source)
        .expect("failed to delete source file");

    operations::delete_file(&destination)
        .expect("failed to delete destination file");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn copy_file_overwrites_existing_destination() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let source = directory.join("source.txt");
    let destination = directory.join("destination.txt");

    operations::create_file(&source)
        .expect("failed to create source file");

    operations::create_file(&destination)
        .expect("failed to create destination file");

    operations::copy_file(
        &source,
        &destination,
        CollisionPolicy::Overwrite,
    )
    .expect("overwrite policy should succeed");

    assert!(source.exists());
    assert!(destination.exists());

    operations::delete_file(&source)
        .expect("failed to delete source file");

    operations::delete_file(&destination)
        .expect("failed to delete destination file");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn rename_fails_when_destination_exists() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let source = directory.join("source.txt");
    let destination = directory.join("destination.txt");

    operations::create_file(&source)
        .expect("failed to create source file");

    operations::create_file(&destination)
        .expect("failed to create destination file");

    let result = operations::rename(
        &source,
        &destination,
        CollisionPolicy::Fail,
    );

    assert!(result.is_err());
    assert!(source.exists());
    assert!(destination.exists());

    operations::delete_file(&source)
        .expect("failed to delete source");

    operations::delete_file(&destination)
        .expect("failed to delete destination");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn rename_skips_existing_destination() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let source = directory.join("source.txt");
    let destination = directory.join("destination.txt");

    operations::create_file(&source)
        .expect("failed to create source file");

    operations::create_file(&destination)
        .expect("failed to create destination file");

    operations::rename(
        &source,
        &destination,
        CollisionPolicy::Skip,
    )
    .expect("skip policy should succeed");

    assert!(source.exists());
    assert!(destination.exists());

    operations::delete_file(&source)
        .expect("failed to delete source");

    operations::delete_file(&destination)
        .expect("failed to delete destination");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn move_fails_when_destination_exists() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let source = directory.join("source.txt");
    let destination = directory.join("destination.txt");

    operations::create_file(&source)
        .expect("failed to create source file");

    operations::create_file(&destination)
        .expect("failed to create destination file");

    let result = operations::move_path(
        &source,
        &destination,
        CollisionPolicy::Fail,
    );

    assert!(result.is_err());
    assert!(source.exists());
    assert!(destination.exists());

    operations::delete_file(&source)
        .expect("failed to delete source");

    operations::delete_file(&destination)
        .expect("failed to delete destination");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn move_skips_existing_destination() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let source = directory.join("source.txt");
    let destination = directory.join("destination.txt");

    operations::create_file(&source)
        .expect("failed to create source file");

    operations::create_file(&destination)
        .expect("failed to create destination file");

    operations::move_path(
        &source,
        &destination,
        CollisionPolicy::Skip,
    )
    .expect("skip policy should succeed");

    assert!(source.exists());
    assert!(destination.exists());

    operations::delete_file(&source)
        .expect("failed to delete source");

    operations::delete_file(&destination)
        .expect("failed to delete destination");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn operations_fail_when_source_does_not_exist() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let missing = directory.join("missing.txt");
    let destination = directory.join("destination.txt");

    let copy_result = operations::copy_file(
        &missing,
        &destination,
        CollisionPolicy::Fail,
    );

    assert!(copy_result.is_err());

    let rename_result = operations::rename(
        &missing,
        &destination,
        CollisionPolicy::Fail,
    );

    assert!(rename_result.is_err());

    let move_result = operations::move_path(
        &missing,
        &destination,
        CollisionPolicy::Fail,
    );

    assert!(move_result.is_err());

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}

#[test]
fn operations_reject_invalid_entry_types() {
    let directory = temporary_test_directory();

    operations::create_directory(&directory)
        .expect("failed to create test directory");

    let file = directory.join("test.txt");
    let subdirectory = directory.join("subdirectory");

    operations::create_file(&file)
        .expect("failed to create test file");

    operations::create_directory(&subdirectory)
        .expect("failed to create test subdirectory");

    // copy_file() must reject directories.
    let copy_result = operations::copy_file(
        &subdirectory,
        &directory.join("copy"),
        CollisionPolicy::Fail,
    );

    assert!(copy_result.is_err());

    // delete_file() must reject directories.
    let delete_file_result =
        operations::delete_file(&subdirectory);

    assert!(delete_file_result.is_err());

    // delete_directory() must reject files.
    let delete_directory_result =
        operations::delete_directory(&file);

    assert!(delete_directory_result.is_err());

    // Cleanup.
    operations::delete_file(&file)
        .expect("failed to delete test file");

    operations::delete_directory(&subdirectory)
        .expect("failed to delete test subdirectory");

    operations::delete_directory(&directory)
        .expect("failed to delete test directory");
}