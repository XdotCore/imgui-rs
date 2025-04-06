#![allow(dead_code)]

use std::{ffi::OsStr, path::{Path, PathBuf}};

const DEFINES: &[(&str, Option<&str>)] = &[
    // Rust `char` is a unicode scalar value, e.g. 32 bits.
    ("IMGUI_USE_WCHAR32", None),
    // Disabled due to linking issues
    ("CIMGUI_NO_EXPORT", None),
    ("IMGUI_DISABLE_WIN32_FUNCTIONS", None),
    ("IMGUI_DISABLE_OSX_FUNCTIONS", None),
];

fn get_all_source_files(path: &str) -> impl Iterator<Item = PathBuf> {
    Path::new(path).read_dir().unwrap().filter_map(|f| {
        match f {
            Ok(f) => {
                let file_type = match f.file_type() {
                    Ok(ft) => ft,
                    Err(_) => return None,
                };
                if file_type.is_file() && ["c", "cpp"].map(|s| OsStr::new(s)).contains(&f.path().extension().unwrap_or_default()) {
                    Some(f.path())
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    })
}

#[cfg(all(feature = "freetype", feature = "use-submodules"))]
fn build_freetype_from_submodule() -> Vec<impl AsRef<std::path::Path>> {
    let freetype_dir = "third-party/freetype";
    let src_dir = &format!("{}/src", freetype_dir);
    let include_dir = format!("{}/include", freetype_dir);

    println!("cargo:rerun-if-changed={}", freetype_dir);

    let mut c_files = vec![
        format!("{}/autofit/autofit.c", src_dir),
        format!("{}/base/ftbase.c", src_dir),
        format!("{}/base/ftbbox.c", src_dir),
        format!("{}/base/ftbdf.c", src_dir),
        format!("{}/base/ftbitmap.c", src_dir),
        format!("{}/base/ftcid.c", src_dir),
        format!("{}/base/ftfstype.c", src_dir),
        format!("{}/base/ftgasp.c", src_dir),
        format!("{}/base/ftglyph.c", src_dir),
        format!("{}/base/ftgxval.c", src_dir),
        format!("{}/base/ftinit.c", src_dir),
        format!("{}/base/ftmm.c", src_dir),
        format!("{}/base/ftotval.c", src_dir),
        format!("{}/base/ftpatent.c", src_dir),
        format!("{}/base/ftpfr.c", src_dir),
        format!("{}/base/ftstroke.c", src_dir),
        format!("{}/base/ftsynth.c", src_dir),
        format!("{}/base/fttype1.c", src_dir),
        format!("{}/base/ftwinfnt.c", src_dir),
        format!("{}/bdf/bdf.c", src_dir),
        format!("{}/bzip2/ftbzip2.c", src_dir),
        format!("{}/cache/ftcache.c", src_dir),
        format!("{}/cff/cff.c", src_dir),
        format!("{}/cid/type1cid.c", src_dir),
        format!("{}/gzip/ftgzip.c", src_dir),
        format!("{}/lzw/ftlzw.c", src_dir),
        format!("{}/pcf/pcf.c", src_dir),
        format!("{}/pfr/pfr.c", src_dir),
        format!("{}/psaux/psaux.c", src_dir),
        format!("{}/pshinter/pshinter.c", src_dir),
        format!("{}/psnames/psnames.c", src_dir),
        format!("{}/raster/raster.c", src_dir),
        format!("{}/sdf/sdf.c", src_dir),
        format!("{}/sfnt/sfnt.c", src_dir),
        format!("{}/smooth/smooth.c", src_dir),
        format!("{}/svg/svg.c", src_dir),
        format!("{}/truetype/truetype.c", src_dir),
        format!("{}/type1/type1.c", src_dir),
        format!("{}/type42/type42.c", src_dir),
        format!("{}/winfonts/winfnt.c", src_dir),
    ];
    #[cfg(not(any(windows, unix)))] {
        c_files.push(format!("{}/base/ftsystem.c", src_dir));
        c_files.push(format!("{}/base/ftdebug.c", src_dir));
    }
    #[cfg(windows)] {
        c_files.push(format!("{}/builds/windows/ftsystem.c", freetype_dir));
        c_files.push(format!("{}/builds/windows/ftdebug.c", freetype_dir));
    }
    #[cfg(unix)] {
        c_files.push(format!("{}/builds/unix/ftsystem.c", freetype_dir));
        c_files.push(format!("{}/base/ftdebug.c", src_dir));
    }

    cc::Build::new()
        .files(c_files)
        .include(&include_dir)
        .define("FT2_BUILD_LIBRARY", None)
        .compile("freetype");

    vec![include_dir]
}

#[cfg(all(feature = "lunasvg", feature = "use-submodules"))]
fn build_lunasvg_from_submodule() -> Vec<impl AsRef<std::path::Path>> {
    let lunasvg_dir = "third-party/lunasvg";
    let src_dir = &format!("{}/source", lunasvg_dir);
    let include_dir = format!("{}/include", lunasvg_dir);
    let plutovg_src_dir = &format!("{}/3rdparty/plutovg", lunasvg_dir);

    println!("cargo:rerun-if-changed={}", src_dir);
    
    cc::Build::new()
        .files(get_all_source_files(src_dir))
        .files(get_all_source_files(plutovg_src_dir))
        .include(&include_dir)
        .include(plutovg_src_dir)
        .cpp(true)
        .flag_if_supported("-std:c++11")
        .flag_if_supported("-std=c++11")
        .define("LUNASVG_BUILD_STATIC", None)
        .compile("lunasvg");

    vec![include_dir]
}

#[cfg(feature = "freetype")]
fn find_freetype() -> Vec<impl AsRef<std::path::Path>> {
    #[cfg(not(any(feature = "use-vcpkg", feature = "use-submodules")))]
    match pkg_config::Config::new().find("freetype2") {
        Ok(freetype) => freetype.include_paths,
        Err(err) => panic!("cannot find freetype: {}", err),
    }
    #[cfg(feature = "use-vcpkg")]
    match vcpkg::find_package("freetype") {
        Ok(freetype) => freetype.include_paths,
        Err(err) => panic!("cannot find freetype: {}", err),
    }
    #[cfg(feature = "use-submodules")]
    build_freetype_from_submodule()
}

#[cfg(feature = "lunasvg")]
fn find_lunasvg() -> Vec<impl AsRef<std::path::Path>> {
    #[cfg(not(any(feature = "use-vcpkg", feature="use-submodules")))]
    match pkg_config::Config::new().find("lunasvg") {
        Ok(lunasvg) => lunasvg.include_paths,
        Err(err) => panic!("cannot find lunasvg: {}", err),
    }
    #[cfg(feature = "use-vcpkg")]
    match vcpkg::find_package("lunasvg") {
        Ok(freetype) => freetype.include_paths,
        Err(err) => panic!("cannot find lunasvg: {}", err),
    }
    #[cfg(feature = "use-submodules")]
    build_lunasvg_from_submodule()
}

// Output define args for compiler
fn main() -> std::io::Result<()> {
    // Root of imgui-sys
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    println!(
        "cargo:THIRD_PARTY={}",
        manifest_dir.join("third-party").display()
    );
    for (key, value) in DEFINES.iter() {
        println!("cargo:DEFINE_{}={}", key, value.unwrap_or(""));
    }

    // Feature flags - no extra dependencies, so these are queried as
    // env-vars to avoid recompilation of build.rs
    let docking_enabled = std::env::var_os("CARGO_FEATURE_DOCKING").is_some();
    let freetype_enabled = std::env::var_os("CARGO_FEATURE_FREETYPE").is_some();
    let wasm_enabled = std::env::var_os("CARGO_FEATURE_WASM").is_some();

    let cimgui_dir = manifest_dir.join(match (docking_enabled, freetype_enabled) {
        (false, false) => "third-party/imgui-master",
        (true, false) => "third-party/imgui-docking",
        (false, true) => "third-party/imgui-master-freetype",
        (true, true) => "third-party/imgui-docking-freetype",
    });

    // For projects like implot-rs we expose the path to our cimgui
    // files, via `DEP_IMGUI_THIRD_PARTY` env-var, so they can build
    // against the same thing
    println!("cargo:THIRD_PARTY={}", cimgui_dir.display());

    // If we aren't building WASM output, bunch of extra stuff to do
    if !wasm_enabled {
        // C++ compiler
        let mut build = cc::Build::new();
        build.cpp(true);

        // imgui uses C++11 stuff from v1.87 onwards
        build.flag_if_supported("-std:c++11");
        build.flag_if_supported("-std=c++11");

        // Set defines for compiler
        for (key, value) in DEFINES.iter() {
            build.define(key, *value);
        }

        // Freetype font rasterizer feature
        #[cfg(feature = "freetype")]
        {
            // Supress warnings:
            // warning: ‘ImFontBuildSrcGlyphFT’ has a field ‘ImFontBuildSrcGlyphFT::Info’ whose type uses the anonymous namespace
            // warning: ‘ImFontBuildSrcDataFT’ has a field ‘ImFontBuildSrcDataFT::Font’ whose type uses the anonymous namespace
            build.flag_if_supported("-Wno-subobject-linkage");

            // Include freetype headers
            for include in find_freetype() {
                build.include(include);
            }

            // Set flag for dear imgui
            build.define("IMGUI_ENABLE_FREETYPE", None);
            build.define("CIMGUI_FREETYPE", None);
            println!("cargo:DEFINE_IMGUI_ENABLE_FREETYPE=");

            // imgui_freetype.cpp needs access to `#include "imgui.h"`.
            // So we include something like '[...]/third-party/imgui-master/imgui/'
            build.include(dbg!(cimgui_dir.join("imgui")));

            #[cfg(feature = "lunasvg")]
            {
                for include in find_lunasvg() {
                    build.include(include);
                }

                build.define("IMGUI_ENABLE_FREETYPE_LUNASVG", None);
                build.define("LUNASVG_BUILD_STATIC", None);
                println!("cargo:DEFINE_IMGUI_ENABLE_FREETYPE_LUNASVG=");
            }
        }

        // Which "all imgui" file to use
        let imgui_cpp = match (docking_enabled, freetype_enabled) {
            (false, false) => "include_imgui_master.cpp",
            (true, false) => "include_imgui_docking.cpp",
            (false, true) => "include_imgui_master_freetype.cpp",
            (true, true) => "include_imgui_docking_freetype.cpp",
        };

        // Set up compiler
        let compiler = build.get_compiler();

        // Avoid the if-supported flag functions for easy cases, as they're
        // kinda costly.
        if compiler.is_like_gnu() || compiler.is_like_clang() {
            build.flag("-fno-exceptions").flag("-fno-rtti");
        }

        // Build imgui lib, suppressing warnings.
        build.warnings(false).file(imgui_cpp).compile("libcimgui.a");
    }
    Ok(())
}
