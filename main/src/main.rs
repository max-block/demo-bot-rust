use core::{error::CoreError, services::bot::UpdateBotParams, Core};

fn main() -> Result<(), CoreError> {
    let mut core: Core = Core::new("mongodb://localhost/demo-rust-rocket-mongodb")?;

    dbg!(&core.bot.bot);
    core.bot.start()?;
    core.bot.update(UpdateBotParams {
        timeout: 1,
        proxy_check_interval: 2,
        work_interval: 3,
    })?;
    dbg!(&core.bot.bot);

    Ok(())
}
