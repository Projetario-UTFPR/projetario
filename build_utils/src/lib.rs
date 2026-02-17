pub fn setup_dev_utils_cfg() {
    let is_test = std::env::var("CKG_RUSTC_TEST").is_ok();
    let has_feature = std::env::var("CARGO_FEATURE_TEST_UTILS").is_ok();

    println!("cargo:rustc-check-cfg=cfg(dev_utils)");

    if is_test || has_feature {
        println!("cargo:rustc-cfg=dev_utils");
    }
}
