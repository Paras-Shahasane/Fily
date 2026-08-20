use fily_core::filesystem::operations;
use fily_core::filesystem::path::FilyPath;
use fily_core::navigation::Navigator;

fn temporary_test_directory() -> FilyPath {
    let unique_id = format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock is before Unix epoch")
            .as_nanos()
    );

    let path = std::env::temp_dir()
        .join(format!("fily-navigation-test-{unique_id}"));

    FilyPath::new(path)
}

#[test]
fn navigator_starts_at_given_directory() {
    let root = temporary_test_directory();

    operations::create_directory(&root)
        .expect("failed to create root directory");

    let navigator =
        Navigator::new(root.clone()).expect("failed to create navigator");

    assert_eq!(navigator.current(), &root);
    assert!(!navigator.can_go_back());
    assert!(!navigator.can_go_forward());

    operations::delete_directory(&root)
        .expect("failed to delete root directory");
}

#[test]
fn navigator_enters_directory() {
    let root = temporary_test_directory();
    let child = root.join("child");

    operations::create_directory(&root)
        .expect("failed to create root directory");

    operations::create_directory(&child)
        .expect("failed to create child directory");

    let mut navigator =
        Navigator::new(root.clone()).expect("failed to create navigator");

    navigator
        .enter(child.clone())
        .expect("failed to enter child directory");

    assert_eq!(navigator.current(), &child);
    assert!(navigator.can_go_back());
    assert!(!navigator.can_go_forward());

    operations::delete_directory(&child)
        .expect("failed to delete child directory");

    operations::delete_directory(&root)
        .expect("failed to delete root directory");
}

#[test]
fn navigator_goes_back_and_forward() {
    let root = temporary_test_directory();
    let child = root.join("child");

    operations::create_directory(&root)
        .expect("failed to create root directory");

    operations::create_directory(&child)
        .expect("failed to create child directory");

    let mut navigator =
        Navigator::new(root.clone()).expect("failed to create navigator");

    navigator
        .enter(child.clone())
        .expect("failed to enter child directory");

    navigator
        .back()
        .expect("failed to navigate back");

    assert_eq!(navigator.current(), &root);
    assert!(!navigator.can_go_back());
    assert!(navigator.can_go_forward());

    navigator
        .forward()
        .expect("failed to navigate forward");

    assert_eq!(navigator.current(), &child);
    assert!(navigator.can_go_back());
    assert!(!navigator.can_go_forward());

    operations::delete_directory(&child)
        .expect("failed to delete child directory");

    operations::delete_directory(&root)
        .expect("failed to delete root directory");
}

#[test]
fn entering_new_directory_clears_forward_history() {
    let root = temporary_test_directory();
    let first = root.join("first");
    let second = root.join("second");

    operations::create_directory(&root)
        .expect("failed to create root directory");

    operations::create_directory(&first)
        .expect("failed to create first directory");

    operations::create_directory(&second)
        .expect("failed to create second directory");

    let mut navigator =
        Navigator::new(root.clone()).expect("failed to create navigator");

    navigator
        .enter(first.clone())
        .expect("failed to enter first directory");

    navigator
        .back()
        .expect("failed to navigate back");

    assert!(navigator.can_go_forward());

    navigator
        .enter(second.clone())
        .expect("failed to enter second directory");

    assert_eq!(navigator.current(), &second);
    assert!(!navigator.can_go_forward());

    operations::delete_directory(&first)
        .expect("failed to delete first directory");

    operations::delete_directory(&second)
        .expect("failed to delete second directory");

    operations::delete_directory(&root)
        .expect("failed to delete root directory");
}

#[test]
fn navigator_goes_to_parent_directory() {
    let root = temporary_test_directory();
    let child = root.join("child");
    let grandchild = child.join("grandchild");

    operations::create_directory(&root)
        .expect("failed to create root directory");

    operations::create_directory(&child)
        .expect("failed to create child directory");

    operations::create_directory(&grandchild)
        .expect("failed to create grandchild directory");

    let mut navigator =
        Navigator::new(grandchild.clone())
            .expect("failed to create navigator");

    navigator
        .parent()
        .expect("failed to navigate to parent");

    assert_eq!(navigator.current(), &child);

    navigator
        .parent()
        .expect("failed to navigate to root");

    assert_eq!(navigator.current(), &root);

    operations::delete_directory(&grandchild)
        .expect("failed to delete grandchild");

    operations::delete_directory(&child)
        .expect("failed to delete child");

    operations::delete_directory(&root)
        .expect("failed to delete root");
}

#[test]
fn navigator_stays_at_filesystem_root() {
    let current = std::env::current_dir()
        .expect("failed to get current directory");

    let mut root = current.clone();

    while let Some(parent) = root.parent() {
        if parent == root {
            break;
        }

        root = parent.to_path_buf();
    }

    let root = FilyPath::new(root);

    let mut navigator =
        Navigator::new(root.clone())
            .expect("failed to create root navigator");

    navigator
        .parent()
        .expect("parent navigation at root should succeed");

    assert_eq!(navigator.current(), &root);
    assert!(!navigator.can_go_back());
}