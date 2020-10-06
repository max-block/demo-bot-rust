use core::{db::CreateWorkerParams, error::CoreError, services::bot::UpdateBotParams, Core};

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

    // core.db.insert_worker(CreateWorkerParams {
    //     name: "w1".to_string(),
    //     source: "https://httpbin.org/ip".to_string(),
    //     proxy: None,
    // })?;

    let res = core.db.find_workers(None, Some(false), 300)?;
    dbg!(res);

    let res = core.db.find_workers_for_work(10, 10)?;
    dbg!(res);

    Ok(())
}
