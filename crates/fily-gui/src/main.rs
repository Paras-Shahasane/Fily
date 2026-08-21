slint::include_modules!();

use fily_core::{
    filesystem::{
        entry::{list_directory, EntryType},
        path::FilyPath,
    },
    navigation::Navigator,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = MainWindow::new()?;

    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .ok_or("Could not determine home directory")?;

    let home_path = FilyPath::new(home);

    let navigator = Navigator::new(home_path.clone())?;

    let entries = list_directory(navigator.current().as_path())?;

    println!("Current directory: {}", navigator.current());

    for entry in &entries {
        let icon = match entry.entry_type() {
            EntryType::Directory => "[DIR]",
            EntryType::File => "[FILE]",
            EntryType::Symlink => "[LINK]",
            EntryType::Other => "[OTHER]",
        };

        println!("{icon} {}", entry.name());
    }

    window.run()?;

    Ok(())
}