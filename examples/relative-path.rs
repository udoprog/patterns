use anyhow::Result;
use relative_path::RelativePath;

fn main() -> Result<()> {
    if cfg!(windows) {
        // this won't work, because the path will be formatted as a `\\?\` path,
        // which does *not* support relative components:
        let path = std::fs::canonicalize(".")?
            .join("..")
            .join("patterns")
            .join("Cargo.toml");
        println!("path: {}", path.display());
        assert!(std::fs::read_to_string(&path).is_err());

        // This however works, because `to_logical_path` operates logically on
        // the base path:
        let base = std::fs::canonicalize(".")?;
        let path = RelativePath::new("../patterns/Cargo.toml").to_logical_path(&base);
        println!("path: {}", path.display());
        assert!(std::fs::read_to_string(&path).is_ok());
    }

    Ok(())
}
