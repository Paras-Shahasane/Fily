use std::fs;

use fily_core::filesystem::{list_directory, EntryType};

fn temporary_test_directory() -> std::path::PathBuf {
    let unique_id = format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock is before Unix epoch")
            .as_nanos()
    );

    std::env::temp_dir().join(format!("fily-listing-test-{unique_id}"))
}

#[test]
fn lists_files_and_directories() {
    let root = temporary_test_directory();

    fs::create_dir_all(&root)
        .expect("failed to create test directory");

    fs::write(root.join("file.txt"), b"hello")
        .expect("failed to create file");

    fs::create_dir(root.join("folder"))
        .expect("failed to create folder");

    let entries =
        list_directory(&root)
            .expect("failed to list directory");

    assert_eq!(entries.len(), 2);

    let file = entries
        .iter()
        .find(|entry| entry.name() == "file.txt")
        .expect("file.txt not found");

    assert_eq!(file.entry_type(), EntryType::File);

    let folder = entries
        .iter()
        .find(|entry| entry.name() == "folder")
        .expect("folder not found");

    assert_eq!(folder.entry_type(), EntryType::Directory);

    fs::remove_dir_all(&root)
        .expect("failed to clean up test directory");
}

#[test]
fn lists_empty_directory() {
    let root = temporary_test_directory();

    fs::create_dir_all(&root)
        .expect("failed to create test directory");

    let entries =
        list_directory(&root)
            .expect("failed to list directory");

    assert!(entries.is_empty());

    fs::remove_dir_all(&root)
        .expect("failed to clean up test directory");
}

#[test]
fn directory_listing_is_sorted_by_name() {
    let root = temporary_test_directory();

    fs::create_dir_all(&root)
        .expect("failed to create test directory");

    fs::write(root.join("zebra.txt"), b"z")
        .expect("failed to create zebra.txt");

    fs::write(root.join("Apple.txt"), b"a")
        .expect("failed to create Apple.txt");

    fs::write(root.join("middle.txt"), b"m")
        .expect("failed to create middle.txt");

    let entries =
        list_directory(&root)
            .expect("failed to list directory");

    let names: Vec<&str> =
        entries.iter().map(|entry| entry.name()).collect();

    assert_eq!(
        names,
        vec!["Apple.txt", "middle.txt", "zebra.txt"]
    );

    fs::remove_dir_all(&root)
        .expect("failed to clean up test directory");
}

#[test]
fn lists_hidden_entries() {
    let root = temporary_test_directory();

    std::fs::create_dir_all(&root)
        .expect("failed to create test directory");

    std::fs::write(root.join(".hidden"), b"hidden")
        .expect("failed to create hidden file");

    std::fs::write(root.join("visible.txt"), b"visible")
        .expect("failed to create visible file");

    let entries =
        list_directory(&root)
            .expect("failed to list directory");

    assert_eq!(entries.len(), 2);

    let hidden = entries
        .iter()
        .find(|entry| entry.name() == ".hidden")
        .expect("hidden entry not found");

    assert!(hidden.is_hidden());

    let visible = entries
        .iter()
        .find(|entry| entry.name() == "visible.txt")
        .expect("visible entry not found");

    assert!(!visible.is_hidden());

    std::fs::remove_dir_all(&root)
        .expect("failed to clean up test directory");
}

#[cfg(unix)]
#[test]
fn lists_symbolic_links() {
    use std::os::unix::fs::symlink;

    let root = temporary_test_directory();

    std::fs::create_dir_all(&root)
        .expect("failed to create test directory");

    let target = root.join("target.txt");
    let link = root.join("link.txt");

    std::fs::write(&target, b"target")
        .expect("failed to create target file");

    symlink(&target, &link)
        .expect("failed to create symbolic link");

    let entries =
        list_directory(&root)
            .expect("failed to list directory");

    let link_entry = entries
        .iter()
        .find(|entry| entry.name() == "link.txt")
        .expect("symbolic link not found");

    assert_eq!(
        link_entry.entry_type(),
        EntryType::Symlink
    );

    std::fs::remove_dir_all(&root)
        .expect("failed to clean up test directory");
}