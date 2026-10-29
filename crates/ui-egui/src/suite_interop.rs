//! Angkor Creative Suite interop: seamless switching between AngkorVector (Illustrator) and AngkorPix (Photoshop).

use std::path::{Path, PathBuf};
use std::process::Command;

/// Attempts to find the AngkorPix executable path.
pub fn find_angkorpix_exe() -> Option<PathBuf> {
    // 1. Check beside current executable
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(dir) = current_exe.parent() {
            let beside = dir.join("angkorpix.exe");
            if beside.is_file() {
                return Some(beside);
            }
            let beside_legacy = dir.join("photocraft.exe");
            if beside_legacy.is_file() {
                return Some(beside_legacy);
            }
        }
    }

    // 2. Check known build paths
    let candidates = [
        r"E:\clone ps\photocraft\target\debug\angkorpix.exe",
        r"E:\clone ps\photocraft\target\release\angkorpix.exe",
        r"..\photocraft\target\debug\angkorpix.exe",
        r"..\photocraft\target\release\angkorpix.exe",
    ];

    for c in &candidates {
        let p = Path::new(c);
        if p.is_file() {
            return Some(p.to_path_buf());
        }
    }

    None
}

/// Launches or brings AngkorPix to the foreground.
pub fn launch_or_focus_angkorpix() {
    if let Some(exe) = find_angkorpix_exe() {
        let _ = Command::new(&exe).spawn();
    } else {
        let photocraft_dir = Path::new(r"E:\clone ps\photocraft");
        if photocraft_dir.is_dir() {
            let _ = Command::new("cargo")
                .args(["run", "-p", "angkorpix"])
                .current_dir(photocraft_dir)
                .spawn();
        }
    }
}

