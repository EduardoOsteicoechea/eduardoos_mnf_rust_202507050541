// // my_ssr_server/build.rs

// use std::env;
// use std::path::{Path, PathBuf};
// use fs_extra::dir::{copy, CopyOptions}; // Keep `copy` for Stage 2
// use fs_extra::copy_items; // Import `copy_items` for copying contents of a directory

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("cargo:rerun-if-changed=static");
//     println!("cargo:rerun-if-changed=html_pages/static");

//     // Get the path to the root of the main crate (my_ssr_server)
//     let main_crate_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

//     // --- Stage 1: Copy static files from html_pages/static to my_ssr_server/static ---
//     let html_pages_static_source_dir = main_crate_manifest_dir.join("html_pages").join("static");
//     let main_crate_static_dest_dir = main_crate_manifest_dir.join("static"); // This is the temporary staging for html_pages' files

//     // Ensure the main crate's static directory exists to receive files
//     std::fs::create_dir_all(&main_crate_static_dest_dir)?;

//     let mut options_stage1 = CopyOptions::new();
//     options_stage1.overwrite = true;
//     options_stage1.buffer_size = 64000; // You might need this or can remove for small files

//     // Instead of `copy` with `copy_inside`, let's try `copy_items` which is better for this.
//     // It takes a Vec of paths to copy, and a single destination.
//     // To copy contents of html_pages_static_source_dir, we need to list its children.
//     let mut items_to_copy_stage1: Vec<PathBuf> = Vec::new();
//     if html_pages_static_source_dir.exists() && html_pages_static_source_dir.is_dir() {
//         for entry in std::fs::read_dir(&html_pages_static_source_dir)? {
//             let entry = entry?;
//             items_to_copy_stage1.push(entry.path());
//         }
//     }

//     if !items_to_copy_stage1.is_empty() {
//         println!(
//             "cargo:warning=Copying html_pages static files contents from {:?} to {:?}",
//             html_pages_static_source_dir, main_crate_static_dest_dir
//         );
//         copy_items(
//             &items_to_copy_stage1, // Pass the list of individual items within the source directory
//             &main_crate_static_dest_dir,
//             &options_stage1,
//         )?;
//     } else {
//         println!(
//             "cargo:warning=html_pages/static directory is empty or not found at {:?}",
//             html_pages_static_source_dir
//         );
//     }

//     // --- Stage 2: Copy combined static files from my_ssr_server/static to target/debug or target/release ---
//     let combined_static_source_dir = main_crate_manifest_dir.join("static"); // This folder now holds everything
//     let target_dir_str = env::var("CARGO_TARGET_DIR")
//         .unwrap_or_else(|_| "target".to_string());
//     let target_dir = PathBuf::from(target_dir_str);
//     let profile = env::var("PROFILE")?;
//     let build_output_dir = target_dir.join(&profile);

//     std::fs::create_dir_all(&build_output_dir)?;

//     let mut options_stage2 = CopyOptions::new();
//     options_stage2.overwrite = true;
//     options_stage2.copy_inside = false; // We want to copy the 'static' folder itself into the target dir

//     if combined_static_source_dir.exists() {
//         println!(
//             "cargo:warning=Copying combined static files from {:?} to {:?}",
//             combined_static_source_dir, build_output_dir
//         );
//         copy( // Use `copy` here as we want to copy the directory itself
//             &combined_static_source_dir,
//             &build_output_dir,
//             &options_stage2,
//         )?;
//     } else {
//         println!(
//             "cargo:warning=Main crate static directory not found at {:?}",
//             combined_static_source_dir
//         );
//     }

//     Ok(())
// }