use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let resources = manifest.join("resources");

    // Dependents generate bindings against these headers. Exported as
    // DEP_NRF_SDC_INCLUDE to dependent build scripts, and reachable from a
    // Makefile via `cargo metadata`.
    println!(
        "cargo::metadata=include={}",
        resources.join("include").display()
    );

    let target = std::env::var("TARGET").unwrap();
    let soc = match target.as_str() {
        "thumbv7em-none-eabi" | "thumbv7em-none-eabihf" => "nrf52",
        // Host builds (doc, clippy, tests) have nothing to link against.
        _ if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("none") => return,
        other => panic!("tock-firmware-nrf-sdc: unsupported target {other}"),
    };
    // nrfxlib ships the nRF52 libraries for three float ABIs. Only the
    // soft-float set is vendored, which is what `thumbv7em-none-eabi` links
    // against.
    let float = "soft-float";

    let roles: Vec<&str> = [
        ("CARGO_FEATURE_PERIPHERAL", "peripheral"),
        ("CARGO_FEATURE_CENTRAL", "central"),
        ("CARGO_FEATURE_MULTIROLE", "multirole"),
    ]
    .iter()
    .filter(|(env, _)| std::env::var_os(env).is_some())
    .map(|(_, role)| *role)
    .collect();
    let role = match roles.as_slice() {
        [role] => *role,
        _ => panic!(
            "tock-firmware-nrf-sdc: select exactly one of the \
             peripheral/central/multirole features, got {roles:?}"
        ),
    };

    let libdir = resources.join("lib").join(soc).join(float);
    println!("cargo::rustc-link-search=native={}", libdir.display());

    // Link order matters: the controller references MPSL, and MPSL references
    // the front-end module shim.
    println!("cargo::rustc-link-lib=static=softdevice_controller_{role}");
    println!("cargo::rustc-link-lib=static=mpsl_fem_common");
    println!("cargo::rustc-link-lib=static=mpsl");

    println!("cargo::rerun-if-changed={}", resources.display());
}
