// ext/levenshtein_rust/build.rs
fn main() {
    let config = rb_sys_build::rb_config();

    if let Some(includedir) = config.get("rubyhdrdir") {
        println!("cargo:include={}", includedir);
    }

    if let Some(archhdrdir) = config.get("rubyarchhdrdir") {
        println!("cargo:include={}", archhdrdir);
    }

    if let Some(libdir) = config.get("libdir") {
        println!("cargo:rustc-link-search=native={}", libdir);
    }

    if let Some(ldflags) = config.get("LDFLAGS") {
        for flag in ldflags.split_whitespace() {
            if flag.starts_with("-l") {
                println!("cargo:rustc-link-lib={}", &flag[2..]);
            }
        }
    }

    println!("cargo:rustc-link-lib=ruby");

    println!("cargo:warning=Ruby version: {}", config.ruby_version_slug());
}
