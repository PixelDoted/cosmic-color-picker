fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CREDIT https://github.com/pop-os/cosmic-files/blob/master/build.rs

    let mut vergen = vergen::EmitBuilder::builder();
    println!("cargo:rerun-if-env-changed=VERGEN_GIT_COMMIT_DATE");
    if std::env::var_os("VERGEN_GIT_COMMIT_DATE").is_none() {
        vergen.git_commit_date();
    }
    println!("cargo:rerun-if-env-changed=VERGEN_GIT_SHA");
    if std::env::var_os("VERGEN_GIT_SHA").is_none() {
        vergen.git_sha(false);
    }

    vergen.fail_on_error().emit()?;
    Ok(())
}
