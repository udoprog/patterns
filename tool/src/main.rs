use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn diff(a: &Path, b: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
    let output = Command::new("diff").arg("-u").args(&[a, b]).output()?;

    if output.status.code() == Some(2) {
        std::io::stdout().write(&output.stderr)?;
        std::io::stdout().write(&output.stdout)?;
        return Err(format!("diff: failed to diff {} and {}", a.display(), b.display()).into());
    }

    fs::write(out, output.stdout)?;
    Ok(())
}

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
    let output = Path::new("snippets");

    if !output.is_dir() {
        fs::create_dir_all(&output)?;
    }

    let snippets = fs::read_dir(output)?;

    for file in snippets {
        let file = file?;
        let path = file.path();

        if path.extension() != Some(OsStr::new("rs")) {
            continue;
        }

        let base = path.file_stem().ok_or_else(|| "missing stem")?;
        let base = base.to_str().ok_or_else(|| "base is invalid utf-8")?;

        println!("Building: {}", path.display());

        let regular = output.join(format!("{}.ll", base));
        let noalias = output.join(format!("{}-noalias.ll", base));

        build(&path, true)?;
        fs::rename("out.ll", &noalias)?;

        build(&path, false)?;
        fs::rename("out.ll", &regular)?;

        let out = output.join(format!("{}.diff", base));
        diff(&regular, &noalias, &out)?;
    }

    Ok(())
}
