use glob::glob;
use image::imageops::FilterType;
use image::GenericImageView;
use std::io::Cursor;

use std::path::Path;
use std::time;
use std::{cmp, fs};

use super::diff::{pixelmatch, Options};

pub fn copy_snaps(report_dir: &Path, dest: &str) {
    println!("Copying snapshots to {}...", dest);
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

pub fn compare_snaps(report_dir: &Path) {
    let original_dir = report_dir.join("original_snapshots");
    let current_dir = report_dir.join("current_snapshots");
    let diff_dir = report_dir.join("diff_snapshots");
    for entry in glob("tests/**/*.png").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // copy
                let original_snap = original_dir.join(path.as_path());
                let current_snap = current_dir.join(path.as_path());
                // .to_str()
                // .map(str::to_string)
                // .unwrap();
                if !original_snap.exists() {
                    println!(
                        "✋ Original snapshot named {} not found",
                        original_snap.display()
                    );
                    continue;
                }
                if !current_snap.exists() {
                    println!(
                        "✋ Latest snapshot named {} not found",
                        original_snap.display()
                    );
                    break;
                }
                let diff_snap = diff_dir.join(path.as_path());
                match create_diff_image(&diff_snap, &original_snap, &current_snap) {
                    Ok(ok) => {
                        if ok == 0 {
                            println!("✅ {}", original_snap.file_name().unwrap());
                        } else {
                            println!("💀 {}", original_snap.file_name().unwrap());
                        }
                    }
                    Err(err) => panic!("failed diff {}", err),
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

pub fn create_diff_image(
    diff_snap: &Path,
    original_snap: &Path,
    current_snap: &Path,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut before = image::open(original_snap).unwrap();
    let mut after = image::open(current_snap).unwrap();

    let mut img_out = Cursor::new(Vec::new());
    let output = match Some(diff_snap) {
        Some(..) => Some(&mut img_out),
        None => None,
    };
    let (width1, height1) = before.dimensions();
    let (width2, height2) = after.dimensions();
    let width = cmp::max(width1, width2);
    let height = cmp::max(height1, height2);
    // println!("w: {:?} h: {:?}", width, height);
    before = before.resize_exact(width, height, FilterType::Triangle);
    after = after.resize_exact(width, height, FilterType::Nearest);
    before.save(original_snap).unwrap();
    after.save(current_snap).unwrap();
    let (width1, height1) = before.dimensions();
    let (width2, height2) = after.dimensions();
    let width = cmp::max(width1, width2);
    let height = cmp::max(height1, height2);
    // println!("1: {:?} 2: {:?}", before.dimensions(), after.dimensions());
    // println!("w: {:?} h: {:?}", width, height);
    let now = time::Instant::now();
    let img1 = fs::read(original_snap)?;
    let img2 = fs::read(current_snap)?;
    let num_diff_pixels = pixelmatch(
        img1.as_slice(),
        img2.as_slice(),
        output,
        Some(width),
        Some(height),
        Some(Options {
            threshold: 0.1,
            ..Default::default()
        }),
    )?;
    // println!("matched in: {}ms", now.elapsed().as_millis());

    let error = ((100.0 * 100.0 * num_diff_pixels as f64) / (width1 as f64 * height1 as f64))
        .round()
        / 100.0;
    // println!("error: {}%", error);

    if let Some(p) = diff_snap.parent() {
        fs::create_dir_all(p)
            .expect("failed to create missing directories before copying diff image snapshot")
    };
    fs::write(diff_snap, img_out.into_inner())?;
    Ok(num_diff_pixels)
}
