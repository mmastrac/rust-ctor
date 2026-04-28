//! Generate docs for crates (generated artifacts).

use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CRATES: &[&str] = &["ctor", "dtor", "link-section"];

fn crate_name(package: &str) -> String {
    package.replace('-', "_")
}

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

fn ensure_jq_available() -> Result<()> {
    if Command::new("jq").arg("--version").output().is_err() {
        bail!("`jq` not found on PATH (required to extract rustdoc JSON)");
    }
    Ok(())
}

fn rustdoc_json(root: &Path, package: &str) -> Result<(tempfile::TempDir, PathBuf)> {
    let temp = tempfile::tempdir().context("create tempdir")?;
    let target_dir = temp.path();

    let mut cargo = Command::new("cargo");
    cargo
        .current_dir(root)
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format json")
        .args(["+nightly", "doc"])
        .args(["-p", package])
        .arg("--quiet")
        .arg("--all-features")
        .arg("--document-private-items")
        .arg("--target-dir")
        .arg(target_dir);
    run(&mut cargo).with_context(|| format!("cargo doc for {}", package))?;

    let json_path = target_dir
        .join("doc")
        .join(format!("{}.json", crate_name(package)));
    if !json_path.exists() {
        bail!("expected rustdoc json at {}", json_path.display());
    }

    Ok((temp, json_path))
}

fn jq(json_path: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("jq")
        .args(args)
        .arg(json_path)
        .output()
        .with_context(|| format!("run jq on {}", json_path.display()))?;

    if !output.status.success() {
        bail!(
            "jq failed for {}: {}",
            json_path.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout).context("jq output not utf-8")
}

fn generated_docs_markdown(json_path: &Path, crate_name: &str) -> Result<String> {
    let module_path = format!("{crate_name}::__generated_docs");
    let filter = format!(
        r#"
. as $r
| ($r.paths | to_entries[] | select((.value.path | join("::")) == "{module_path}") | .key) as $id
| $r.index[$id].docs
"#
    );
    jq(json_path, &["-r", &filter])
}

fn crate_root_module_json(json_path: &Path) -> Result<String> {
    // rustdoc JSON contains a `root` item id pointing at the crate root module.
    // We want the crate-level docs markdown, not the full JSON entry.
    jq(json_path, &["-r", ".index[(.root | tostring)].docs"])
}

fn write_generated_docs(root: &Path, package: &str, docs: &str) -> Result<()> {
    let out = root.join(package).join("docs").join("GENERATED.md");
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(&out, docs).with_context(|| format!("write {}", out.display()))?;
    Ok(())
}

fn write_readme_markdown(root: &Path, package: &str, docs: &str) -> Result<()> {
    let out = root.join(package).join("README.md");

    // Strip "invisible" lines from examples: remove any line where left-trimmed text starts with
    // "# ", but only when inside fenced code blocks (```).
    let mut in_fence = false;
    let mut rendered_lines = Vec::new();
    for line in docs.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            rendered_lines.push(line);
            continue;
        }

        if in_fence && trimmed.starts_with("# ") {
            continue;
        }
        rendered_lines.push(line);
    }

    let mut rendered = rendered_lines.join("\n");
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }

    fs::write(&out, rendered).with_context(|| format!("write {}", out.display()))?;
    Ok(())
}

fn main() -> Result<()> {
    let root = workspace_root()?;

    ensure_jq_available()?;

    for package in CRATES {
        let crate_name = crate_name(package);
        // Pass 1: extract just the `__generated_docs` markdown.
        let (_temp, json_path) = rustdoc_json(&root, package)
            .with_context(|| format!("rustdoc json for {}", package))?;
        let docs = generated_docs_markdown(&json_path, &crate_name)
            .with_context(|| format!("extract __generated_docs for {}", package))?;
        if !docs.trim().is_empty() {
            write_generated_docs(&root, package, &docs)
                .with_context(|| format!("write __generated_docs for {}", package))?;
        }

        // Pass 2: rerun rustdoc JSON and write the crate root docs markdown into README.
        let (_temp, json_path) = rustdoc_json(&root, package)
            .with_context(|| format!("rustdoc json for {}", package))?;
        let root_docs = crate_root_module_json(&json_path)
            .with_context(|| format!("extract crate root docs for {}", package))?;

        if !root_docs.trim().is_empty() {
            write_readme_markdown(&root, package, &root_docs)
                .with_context(|| format!("write README markdown for {}", package))?;
        }
    }

    Ok(())
}
