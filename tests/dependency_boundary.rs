use std::{path::PathBuf, process::Command};

struct WorkspaceManifest {
    path: PathBuf,
}

impl WorkspaceManifest {
    fn from_environment() -> Self {
        Self {
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        }
    }

    fn cargo_tree(&self, arguments: &[&str]) -> String {
        let output = Command::new(env!("CARGO"))
            .arg("tree")
            .args(arguments)
            .current_dir(&self.path)
            .output()
            .expect("cargo tree runs");
        assert!(
            output.status.success(),
            "cargo tree failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).expect("cargo tree stdout is UTF-8")
    }
}

#[test]
fn default_query_surface_has_no_nota_dependency() {
    let manifest = WorkspaceManifest::from_environment();
    let tree = manifest.cargo_tree(&["--edges", "normal", "--no-default-features"]);

    assert!(
        !tree.contains("\n├── nota ") && !tree.contains("\n└── nota "),
        "default query dependency tree must not contain nota:\n{tree}"
    );
}

#[test]
fn nota_text_feature_opts_into_nota_dependency() {
    let manifest = WorkspaceManifest::from_environment();
    let tree = manifest.cargo_tree(&[
        "--edges",
        "normal",
        "--no-default-features",
        "--features",
        "nota-text",
    ]);

    assert!(
        tree.contains("\n├── nota ") || tree.contains("\n└── nota "),
        "nota-text feature dependency tree must contain nota:\n{tree}"
    );
}
