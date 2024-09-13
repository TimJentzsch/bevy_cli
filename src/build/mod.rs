use crate::{
    external_cli::{cargo, wasm_bindgen},
    mainfest::package_name,
    web,
};

pub use self::args::BuildArgs;

mod args;

pub fn build(args: &BuildArgs) -> anyhow::Result<()> {
    if args.is_web() {
        web::ensure_setup()?;
    }

    let cargo_args = args.cargo_args_builder();

    if args.is_web() {
        println!("Building for WASM...");
        cargo::build::command().args(cargo_args).status()?;

        println!("Bundling for the web...");
        wasm_bindgen::bundle(&package_name()?, args.is_release())
            .expect("Failed to bundle for the web");
    } else {
        cargo::build::command().args(cargo_args).status()?;
    }

    Ok(())
}
