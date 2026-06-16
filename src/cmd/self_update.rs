use anyhow::Result;

pub fn self_update() -> Result<()> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("23prime")
        .repo_name("backlog-cli")
        .bin_name("bl")
        .show_download_progress(true)
        .current_version(self_update::cargo_crate_version!())
        .build()?
        .update()?;

    if status.updated() {
        anstream::println!("Updated to {}", status.version());
    } else {
        anstream::println!("Already up to date ({})", status.version());
    }

    Ok(())
}
