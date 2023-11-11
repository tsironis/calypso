use glob::glob;
use std::{fs, path::PathBuf};

pub fn copy_snaps(report_dir: &PathBuf, dest: &str) {
    // TODO git checkoutargs.branch
    println!("Copying original snapshots...");
    let og_dir = report_dir.join(dest);
    for entry in glob("tests/**/*.png").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // copy
                let dest = og_dir.join(path.as_path());
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
