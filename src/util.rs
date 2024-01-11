use askama::Template;
use glob::glob;
use image::imageops::FilterType;
use image::GenericImageView;
use std::io::Cursor;

use std::path::{Path, PathBuf};
use std::time;
use std::{cmp, fs};

use crate::Cli;
use anyhow::{Context, Result};

use super::diff::{pixelmatch, Options};

const GLOB_PATTERN: &str = "tests/**/*.png";

pub fn copy_snaps(report_dir: &Path, dest: &str) -> Result<()> {
    println!("Copying snapshots to {}...", dest);
    let og_dir = report_dir.join(dest);
    let entries = glob(GLOB_PATTERN)
        .with_context(|| format!("Failed to read glob pattern {}", GLOB_PATTERN))?;
    for entry in entries {
        let path = entry.with_context(|| {
            format!(
                "Failed to process entry in glob iterator for pattern {}",
                GLOB_PATTERN
            )
        })?;
        // copy
        let dest = og_dir.join(path.as_path());
        if let Some(p) = dest.parent() {
            fs::create_dir_all(p).with_context(|| {
                format!(
                    "failed to create missing directories before copying snapshot to {:?}",
                    p
                )
            })?;
        };
        fs::copy(&path, &dest)
            .with_context(|| format!("failed to copy snapshot from {:?} to {:?}", path, dest))?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct Snap {
    name: String,
    original: PathBuf,
    current: PathBuf,
    diff: PathBuf,
    error: f64,
    num_diff_pixels: usize,
}

pub fn compare_snaps(report_dir: &Path, args: &Cli) -> Result<Vec<Snap>, anyhow::Error> {
    let original_dir = report_dir.join("original_snapshots");
    let current_dir = report_dir.join("current_snapshots");
    let diff_dir = report_dir.join("diff_snapshots");
    let mut snaps: Vec<Snap> = Vec::new();
    for entry in glob("tests/**/*.png").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // copy
                let original = original_dir.join(path.as_path());
                let current = current_dir.join(path.as_path());
                if !original.exists() {
                    println!(
                        "✋ Original snapshot named {} not found",
                        original.display()
                    );
                    continue;
                }
                if !current.exists() {
                    println!("✋ Latest snapshot named {} not found", original.display());
                    break;
                }
                let diff = diff_dir.join(path.as_path());
                match create_diff_image(&diff, &original, &current, &args) {
                    Ok((error, num_diff_pixels)) => {
                        let file_name = original.strip_prefix(&original_dir)?;
                        if num_diff_pixels == 0 {
                            println!("✅ {}", file_name.display());
                        } else {
                            println!("💀 {}", file_name.display());
                        }
                        let snap = Snap {
                            name: original
                                .file_name()
                                .unwrap()
                                .to_owned()
                                .into_string()
                                .unwrap(),
                            original: original.strip_prefix(report_dir).unwrap().to_path_buf(),
                            current: current.strip_prefix(report_dir).unwrap().to_path_buf(),
                            diff: diff.strip_prefix(report_dir).unwrap().to_path_buf(),
                            error,
                            num_diff_pixels,
                        };
                        snaps.push(snap)
                    }
                    Err(err) => panic!("failed diff {}", err),
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    return Ok(snaps);
}

pub fn create_diff_image(
    diff_snap: &Path,
    original_snap: &Path,
    current_snap: &Path,
    args: &Cli,
) -> Result<(f64, usize)> {
    let mut before = image::open(original_snap)?;
    let mut after = image::open(current_snap)?;
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
    before.save(original_snap)?;
    after.save(current_snap)?;
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
    if args.verbose {
        println!("   matched in: {}ms", now.elapsed().as_millis())
    }

    let error = ((100.0 * 100.0 * num_diff_pixels as f64) / (width1 as f64 * height1 as f64))
        .round()
        / 100.0;

    if let Some(p) = diff_snap.parent() {
        fs::create_dir_all(p).with_context(|| {
            format!(
                "failed to create missing directories before copying diff image snapshot to {}",
                p.display()
            )
        })?
    };
    fs::write(diff_snap, img_out.into_inner())?;
    Ok((error, num_diff_pixels))
}

#[derive(Template)]
#[template(path = "template.html")]
pub struct DiffTemplate<'a> {
    name: &'a str,
    snaps: Vec<Snap>,
    current_branch: String,
    dest_branch: String,
}

pub fn create_report(
    report_dir: &Path,
    snaps: Vec<Snap>,
    dest_branch: String,
    current_branch: String,
) -> Result<()> {
    let report_file = report_dir.join("index.html");
    let diff_template = DiffTemplate {
        name: "Calypso",
        snaps,
        current_branch,
        dest_branch,
    }; // instantiate your struct
    fs::write(report_file, diff_template.render().unwrap())?;
    Ok(())
}
