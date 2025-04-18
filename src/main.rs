//---------------------------------------------------------------------------//
// Copyright (c) 2022-2025 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Workshopper project,
// which can be found here: https://github.com/Frodo45127/workshopper.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

//! This is a small CLI tool to interact with the Steam Workshop.
//!
//! While initially designed for Total War games... may work with any other game.

// Disabled `Clippy` linters, with the reasons why they were disabled.
#![allow(
    clippy::type_complexity,                // Disabled due to useless warnings.
    clippy::too_many_arguments              // Disabled because it gets annoying really quick.
)]

use anyhow::Result;
use clap::Parser;
use steamworks::PublishedFileId;

use std::path::PathBuf;
use std::process::exit;

use rpfm_lib::integrations::log::*;

use crate::app::{Cli, Commands};

#[cfg(target_os = "windows")]
const STEAM_PROCESS_NAME: &str = "steam.exe";

#[cfg(any(target_os = "linux", target_os = "macos"))]
const STEAM_PROCESS_NAME: &str = "steam";

mod app;
mod commands;

/// Guess you know what this function does....
fn main() {

    let logger = Logger::init(&PathBuf::from("."), true, true, release_name!());
    if logger.is_err() {
        warn!("Logging initialization has failed. No logs will be saved.");
    }

    if !is_steam_running() {
        error!("Steam is not running. Make sure Steam is running or this tool will not work as expected.");
        exit(1)
    }

    // Parse the entire cli command.
    let cli = Cli::parse();
    info!("{:?}", cli.command);

    // Execute the commands.
    let (result, wait): (Result<()>, bool) = match cli.command {
        Commands::DownloadSubscribedItems { steam_id, published_file_ids } => (crate::commands::ugc::download_subscribed_mods(steam_id, published_file_ids), true),
        Commands::GetPublishedFileDetails { steam_id, published_file_ids, ipc_channel } => (crate::commands::ugc::published_file_details(steam_id, &published_file_ids, &ipc_channel), false),
        Commands::Launch { base64, steam_id, command } => (crate::commands::launch_game(base64, steam_id, &command), false),
        Commands::Upload { base64, steam_id, file_path, title, description, tags, changelog, visibility } => (crate::commands::ugc::upload(base64, steam_id, &file_path, &title, &description, &tags, &changelog, &visibility), true),
        Commands::Update { base64, steam_id, published_file_id, file_path, title, description, tags, changelog, visibility } => (crate::commands::ugc::update(None, None, base64, PublishedFileId(published_file_id), steam_id, &file_path, &title, &description, &tags, &changelog, &visibility), true),
        Commands::UserId { steam_id, ipc_channel } => (crate::commands::user_id(steam_id, &ipc_channel), false)
    };

    // Output the result of the commands, then give people 60 seconds to read them before exiting.
    match result {
        Ok(_) => {
            info!("Done. This terminal will close itself in 60 seconds to give you some time to read the log, but if you want, you can close it now.");
            if cfg!(debug_assertions) || wait {
               std::thread::sleep(std::time::Duration::from_millis(60000));
            }

            exit(0)
        },
        Err(error) => {
            error!("{}", error.to_string());
            info!("This terminal will close itself in 60 seconds to give you some time to read the log, but if you want, you can close it now.");
            if cfg!(debug_assertions) || wait {
               std::thread::sleep(std::time::Duration::from_millis(60000));
            }

            exit(1);
        },
    }
}

pub fn is_steam_running() -> bool {
    let refresh_kind = sysinfo::RefreshKind::everything()
        .with_processes(sysinfo::ProcessRefreshKind::everything());
    let sys = sysinfo::System::new_with_specifics(refresh_kind);
    sys.processes_by_exact_name(STEAM_PROCESS_NAME.as_ref()).count() > 0
}
