use crate::{db, error::Error};
use auto_launch::{AutoLaunch, AutoLaunchBuilder};

pub fn set_auto_launch(new_state: bool) -> Result<(), Error> {
    let current_state = db::get_auto_launch()?;
    if current_state == new_state {
        return Ok(());
    }
    set_state(new_state)?;
    db::set_auto_launch(new_state)
}

pub fn init() -> Result<(), Error> {
    let current_state = db::get_auto_launch()?;
    println!("auto_launch: {}", current_state);
    set_state(current_state)
}

fn set_state(new_state: bool) -> Result<(), Error> {
    let auto = build()?;
    if auto.is_enabled()? == new_state {
        return Ok(());
    }

    match new_state {
        true => auto.enable()?,
        false => auto.disable()?,
    }
    Ok(())
}

fn build() -> Result<AutoLaunch, Error> {
    let app_name = db::get_app_name()?;
    Ok(AutoLaunchBuilder::new()
        .set_app_name(&app_name)
        .set_app_path(
            std::env::current_exe()?
                .to_str()
                .ok_or(Error::UnexpectedError(
                    "unkown conversion error".to_string(),
                ))?,
        )
        .build()?)
}
