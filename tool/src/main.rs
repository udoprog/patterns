use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

fn build(path: &Path, noalias: bool) -> Result<(), Box<dyn Error>> {
    let noalias = if noalias {
        "mutable-noalias=yes"
    } else {
        "mutable-noalias=no"
    };

    // NB: -O is necessary for mutable-noalias to kick in.
    let status = Command::new("rustc")
        .args(&[
            "+nightly",
            "--crate-name",
            "out",
            "--crate-type=rlib",
            "-O",
            "-Z",
            noalias,
            "--emit",
            "llvm-ir",
        ])
        .arg(path)
        .status()?;

    if !status.success() {
        return Err(format!("rustc: failed to compile {}", path.display()).into());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let snippets = fs::read_dir("snippets")?;

    let output = Path::new("snippets-llvm-ir");

    if !output.is_dir() {
        fs::create_dir_all(&output)?;
    }

    for file in snippets {
        let file = file?;
        let path = file.path();

        let base = path.file_stem().ok_or_else(|| "missing stem")?;
        let base = base.to_str().ok_or_else(|| "base is invalid utf-8")?;

        println!("Building: {}", path.display());

        build(&path, true)?;
        fs::rename("out.ll", output.join(format!("{}-noalias.ll", base)))?;

        build(&path, false)?;
        fs::rename("out.ll", output.join(format!("{}.ll", base)))?;
    }

    Ok(())
}
