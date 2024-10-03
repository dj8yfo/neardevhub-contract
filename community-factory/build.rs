use cargo_near_build::{bon, extended};
use cargo_near_build::{BuildImplicitEnvOpts, BuildOpts};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!(
        "cargo:warning={}",
        format!(
            "`devhub-community-factory` build script working dir: {:?}",
            std::env::current_dir().expect("get current dir")
        )
    );

    // directory of target `devhub-community` sub-contract's crate
    let workdir = "../community";
    // unix path to target `devhub-community` sub-contract's crate from root of the repo
    let nep330_contract_path = "community";

    let env = ["KEY", "GOOGLE_QUERY"]
        .into_iter()
        .filter(|key| std::env::var(key).is_ok())
        .map(|key| (key.to_string(), std::env::var(key).unwrap()))
        .collect::<Vec<_>>();

    let build_opts = BuildOpts::builder().env(env).build();

    let pwd = std::env::current_dir().expect("get pwd");
    // a distinct target is needed to avoid deadlock during build
    let distinct_target = pwd.join("../target/build-rs-community-for-community-factory");
    let stub_path = pwd.join("../target/community-stub.bin");

    let build_implicit_env_opts = BuildImplicitEnvOpts::builder()
        .nep330_contract_path(nep330_contract_path)
        .cargo_target_dir(distinct_target.to_string_lossy())
        .build();

    let build_script_opts = extended::BuildScriptOpts::builder()
        .rerun_if_changed_list(bon::vec![
            "../discussions", // transitive dependecy of `devhub-community` contract
            workdir,
            "Cargo.toml",
            "../Cargo.lock",
        ])
        .build_skipped_when_env_is(vec![
            // shorter build for `cargo check`
            ("PROFILE", "debug"),
            (cargo_near_build::env_keys::BUILD_RS_ABI_STEP_HINT, "true"),
        ])
        .stub_path(stub_path.to_string_lossy())
        .result_env_key("BUILD_RS_SUB_BUILD_DEVHUB-COMMUNITY")
        .build();

    let extended_opts = extended::BuildOptsExtended::builder()
        .workdir(workdir)
        .build_opts(build_opts)
        .build_implicit_env_opts(build_implicit_env_opts)
        .build_script_opts(build_script_opts)
        .build();
    cargo_near_build::extended::build(extended_opts)?;
    Ok(())
}
