//! Generate docs for crates.

use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct CrateSpec {
    /// Cargo package name (for `cargo doc -p`).
    package: &'static str,
    /// Rust crate name (for rustdoc JSON file + path, uses underscores).
    crate_name: &'static str,
    /// Path to crate directory within the workspace.
    dir: &'static str,
}

const CRATES: &[CrateSpec] = &[
    CrateSpec {
        package: "ctor",
        crate_name: "ctor",
        dir: "ctor",
    },
    CrateSpec {
        package: "dtor",
        crate_name: "dtor",
        dir: "dtor",
    },
    CrateSpec {
        package: "link-section",
        crate_name: "link_section",
        dir: "link-section",
    },
];

fn workspace_root() -> Result<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| anyhow!("xtask has no parent directory"))?
        .to_path_buf();
    Ok(root)
}

fn run(cmd: &mut Command) -> Result<()> {
    let status = cmd
        .status()
        .with_context(|| format!("failed to run {:?}", cmd))?;
    if !status.success() {
        bail!("command {:?} failed with {}", cmd, status);
    }
    Ok(())
}

fn generate_for_crate(root: &Path, spec: &CrateSpec) -> Result<String> {
    let temp = tempfile::tempdir().context("create tempdir")?;
    let target_dir = temp.path();

    // Generate rustdoc JSON into the temp target dir.
    let mut cargo = Command::new("cargo");
    cargo
        .current_dir(root)
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format json")
        .args(["+nightly", "doc"])
        .args(["-p", spec.package])
        .arg("--all-features")
        .arg("--document-private-items")
        .arg("--target-dir")
        .arg(target_dir);
    run(&mut cargo).with_context(|| format!("cargo doc for {}", spec.package))?;

    let json_path = target_dir
        .join("doc")
        .join(format!("{}.json", spec.crate_name));
    if !json_path.exists() {
        bail!("expected rustdoc json at {}", json_path.display());
    }

    // Extract the docs for the __generated_docs module.
    let module_path = format!("{}::__generated_docs", spec.crate_name);
    let jq_filter = format!(
        r#"
  . as $r
  | ($r.paths | to_entries[] | select((.value.path | join("::")) == "{module_path}") | .key) as $id
  | $r.index[$id].docs
"#
    );

    let output = Command::new("jq")
        .arg("-r")
        .arg(jq_filter)
        .arg(&json_path)
        .output()
        .context("run jq")?;

    if !output.status.success() {
        bail!(
            "jq failed for {} ({}): {}",
            spec.package,
            json_path.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let docs = String::from_utf8(output.stdout).context("jq output not utf-8")?;
    Ok(docs)
}

fn write_outputs(root: &Path, spec: &CrateSpec, docs: &str) -> Result<()> {
    let crate_dir = root.join(spec.dir);

    let generated_md = crate_dir.join("docs").join("GENERATED.md");
    if let Some(parent) = generated_md.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(&generated_md, docs).with_context(|| format!("write {}", generated_md.display()))?;

    // For README, strip "invisible" lines from examples:
    // remove any line where left-trimmed text starts with "# ".
    let mut readme_docs = docs
        .lines()
        .filter(|line| !line.trim_start().starts_with("# "))
        .collect::<Vec<_>>()
        .join("\n");
    if docs.ends_with('\n') {
        readme_docs.push('\n');
    }

    let readme_md = crate_dir.join("README.md");
    fs::write(&readme_md, readme_docs).with_context(|| format!("write {}", readme_md.display()))?;

    Ok(())
}

fn main() -> Result<()> {
    let root = workspace_root()?;

    // Ensure jq is available early with a friendly message.
    let jq_ok = Command::new("jq").arg("--version").output();
    if jq_ok.is_err() {
        bail!("`jq` not found on PATH (required to extract rustdoc JSON docs)");
    }

    for spec in CRATES {
        let mut docs = generate_for_crate(&root, spec)
            .with_context(|| format!("generate docs for {}", spec.package))?;

        // Some crates may not generate a __generated_docs module yet; in that case
        // keep existing README content rather than clobbering it with empty output.
        if docs.trim().is_empty() {
            let readme_path = root.join(spec.dir).join("README.md");
            docs = fs::read_to_string(&readme_path)
                .with_context(|| format!("read fallback {}", readme_path.display()))?;
            if docs.trim().is_empty() {
                bail!(
                    "no docs extracted for {} and fallback README is empty",
                    spec.package
                );
            }
        }

        write_outputs(&root, spec, &docs)
            .with_context(|| format!("write docs for {}", spec.package))?;
    }

    Ok(())
}
