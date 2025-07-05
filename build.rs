// my_ssr_server/build.rs
// This script runs automatically before your main application is compiled.

use std::env;
use std::path::{Path, PathBuf};
use fs_extra::dir::{copy, CopyOptions};
use fs_extra::copy_items; // Import `copy_items` for copying contents of a directory

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Tell Cargo to re-run this build script if relevant directories change.
    println!("cargo:rerun-if-changed=static");
    println!("cargo:rerun-if-changed=html_pages/static");
    println!("cargo:rerun-if-changed=pages_components/src/components/"); // New: Watch component directories

    // Get the path to the root of the main crate (my_ssr_server)
    let main_crate_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    // Destination for all static files (temporary staging area)
    let main_crate_static_dest_dir = main_crate_manifest_dir.join("static");
    std::fs::create_dir_all(&main_crate_static_dest_dir)?; // Ensure it exists

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.buffer_size = 64000;

    // --- Stage 1: Copy static files from pages_components/src/components/*/ to my_ssr_server/static ---
    let components_base_source_dir = main_crate_manifest_dir.join("pages_components").join("src").join("components");
    let mut component_static_files_to_copy: Vec<PathBuf> = Vec::new();

    if components_base_source_dir.exists() && components_base_source_dir.is_dir() {
        println!("cargo:warning=Collecting static files from pages_components...");
        for component_entry in std::fs::read_dir(&components_base_source_dir)? {
            let component_path = component_entry?.path();
            if component_path.is_dir() {
                // This is a component directory (e.g., button_component)
                println!("cargo:warning=  Processing component directory: {:?}", component_path);
                for file_entry in std::fs::read_dir(&component_path)? {
                    let file_path = file_entry?.path();
                    if file_path.is_file() {
                        if let Some(extension) = file_path.extension() {
                            if extension == "html" || extension == "css" || extension == "js" {
                                component_static_files_to_copy.push(file_path);
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!(
            "cargo:warning=pages_components/src/components directory not found at {:?}",
            components_base_source_dir
        );
    }

    if !component_static_files_to_copy.is_empty() {
        println!(
            "cargo:warning=Copying pages_components static files to {:?}",
            main_crate_static_dest_dir
        );
        // `copy_items` copies individual files from a list to a destination directory.
        // This ensures no nested component directories in the main static folder.
        copy_items(
            &component_static_files_to_copy,
            &main_crate_static_dest_dir,
            &copy_options,
        )?;
    } else {
        println!("cargo:warning=No static files found in pages_components to copy.");
    }


    // --- Stage 2: Copy static files from html_pages/static to my_ssr_server/static ---
    let html_pages_static_source_dir = main_crate_manifest_dir.join("html_pages").join("static");

    let mut html_page_static_files_to_copy: Vec<PathBuf> = Vec::new();
    if html_pages_static_source_dir.exists() && html_pages_static_source_dir.is_dir() {
        for entry in std::fs::read_dir(&html_pages_static_source_dir)? {
            let entry = entry?;
            html_page_static_files_to_copy.push(entry.path());
        }
    }

    if !html_page_static_files_to_copy.is_empty() {
        println!(
            "cargo:warning=Copying html_pages static files to {:?}",
            main_crate_static_dest_dir
        );
        copy_items(
            &html_page_static_files_to_copy,
            &main_crate_static_dest_dir,
            &copy_options,
        )?;
    } else {
        println!(
            "cargo:warning=html_pages/static directory is empty or not found at {:?}",
            html_pages_static_source_dir
        );
    }


    // --- Stage 3: Copy combined static files from my_ssr_server/static to target/debug or target/release ---
    let combined_static_source_dir = main_crate_static_dest_dir; // This is now the source for the final copy
    let target_dir_str = env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string());
    let target_dir = PathBuf::from(target_dir_str);
    let profile = env::var("PROFILE")?;
    let build_output_dir = target_dir.join(&profile);

    std::fs::create_dir_all(&build_output_dir)?;

    // For Stage 3, we want to copy the 'static' folder itself into the target.
    // So, 'copy_inside' should be false.
    copy_options.copy_inside = false; // Re-use and modify options

    if combined_static_source_dir.exists() {
        println!(
            "cargo:warning=Copying combined static files from {:?} to {:?}",
            combined_static_source_dir, build_output_dir
        );
        copy( // Use `copy` here as we want to copy the directory itself
            &combined_static_source_dir,
            &build_output_dir,
            &copy_options,
        )?;
    } else {
        println!(
            "cargo:warning=Main crate static directory not found at {:?}",
            combined_static_source_dir
        );
    }

    Ok(())
}