//---------------------------------------------------------------------------//
// Copyright (c) 2022-2025 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Workshopper project,
// which can be found here: https://github.com/Frodo45127/workshopper.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

//!Build script for the Workshopper.

/// Windows Build Script.
#[cfg(target_os = "windows")]
fn main() {

    // Copy the dll needed for workshopper to work. Allow an error for cases like using this as a submodule.
    let _ = std::fs::copy("./3rdparty/steam_api64.dll", "./target/debug/steam_api64.dll");
    println!("cargo:rustc-link-lib=dylib=steam_api64");
}

/// Linux Build Script.
#[cfg(target_os = "linux")]
fn main() {}

/// MacOS Build Script.
#[cfg(target_os = "macos")]
fn main() {}
