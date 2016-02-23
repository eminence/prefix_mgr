extern crate setenv;
extern crate clap;

use setenv::{get_shell, Shell};
use clap::{Arg, App, SubCommand};
use std::env::{var, var_os};
use std::path::PathBuf;


fn main() {
    let s = get_shell();
    let stderr = std::io::stderr();

    let matches = App::new("Prefix Manager")
        .version("0.0.1")
        .arg(Arg::with_name("name")
             .long("name")
             .help("Name of prefix")
             .takes_value(true))
        .get_matches();


    let prefix_name = matches.value_of("name").unwrap();

    let prefix_root = var_os("PREFIX_ROOT").unwrap_or_else(|| panic!("Please set PREFIX_ROOT"));

    let prefix_path = PathBuf::from(prefix_root);

    let ldpath = prefix_path.join("lib");
    let ld_flags = if let Ok(ld) = var("LDFLAGS") {
        format!("-L{} {}", ldpath.display(), ld)
    } else {
        format!("-L{}", ldpath.display())
    };
    
    
    let c = prefix_path.join("include");
    let cflags = if let Ok(cf) = var("CFLAGS") {
        format!("-I{} {}", c.display(), cf)
    } else {
        format!("-I{}", c.display())
    };

    let cxxflags = if let Ok(cf) = var("CXXFLAGS") {
        format!("-I{} {}", c.display(), cf)
    } else {
        format!("-I{}", c.display())
    };


    let pkg_config = prefix_path.join("lib").join("pkgconfig");
    let mut pkg_config_path = s.split_env("PKG_CONFIG_PATH");
    pkg_config_path.insert(0, pkg_config.into_os_string());


    let aclocal = prefix_path.join("share").join("aclocal");
    let aclocal_flags = if let Ok(ac) = var("ACLOCAL_FLAGS") {
        format!("-I{} {}", c.display(), ac)
    } else {
        format!("-I{}", c.display())
    };

    let ldpath = prefix_path.join("lib");
    let mut ld_library_path = s.split_env("LD_LIBRARY_PATH");
    ld_library_path.insert(0, ldpath.into_os_string());
    
    let man = prefix_path.join("share").join("man");
    let mut manpath = s.split_env("MANPATH");
    manpath.insert(0, man.into_os_string());
    
    let bin = prefix_path.join("bin");
    let mut path = s.split_env("PATH");
    path.insert(0, bin.into_os_string());

    s.setenv("LDFLAGS", ld_flags);
    s.setenv("CFLAGS", cflags);
    s.setenv("CXXFLAGS", cxxflags);
    s.setenv_list("PKG_CONFIG_PATH", pkg_config_path);
    s.setenv("ACLOCAL_FLAGS", aclocal_flags);
    s.setenv_list("LD_LIBRARY_PATH", ld_library_path);
    s.setenv_list("MANPATH", manpath);
    s.setenv_list("PATH", path);
}
