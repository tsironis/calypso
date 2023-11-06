use glob::glob;
use std::{fs, path::PathBuf};

pub fn copy_og_snaps(report_dir: PathBuf, base_branch: String) {
    // TODO git checkoutargs.branch
    println!("Copying original snapshots...");
    for entry in glob("tests/**/*.png").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // copy
                let dest = report_dir.join(path.as_path());
                if let Some(p) = dest.parent() {
                    fs::create_dir_all(p)
                        .expect("failed to create missing directories before copying snapshot")
                };
                fs::copy(path, dest).expect("failed to copy snapshot");
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
