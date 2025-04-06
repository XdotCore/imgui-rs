#![allow(dead_code)]

const DEFINES: &[(&str, Option<&str>)] = &[
    // Rust `char` is a unicode scalar value, e.g. 32 bits.
    ("IMGUI_USE_WCHAR32", None),
    // Disabled due to linking issues
    ("CIMGUI_NO_EXPORT", None),
    ("IMGUI_DISABLE_WIN32_FUNCTIONS", None),
    ("IMGUI_DISABLE_OSX_FUNCTIONS", None),
];

#[cfg(all(feature = "freetype", feature = "use-submodules"))]
fn build_freetype_from_git() -> Vec<impl AsRef<std::path::Path>> {
    let src_dir = "third-party/freetype";
    let include_dir = format!("{}/include", src_dir);

    println!("cargo:rerun-if-changed={}", src_dir);

    let c_files = [
        &format!("{}/src/base/ftsystem.c", src_dir),
        &format!("{}/src/base/ftinit.c", src_dir),
        &format!("{}/src/base/ftdebug.c", src_dir),
        &format!("{}/src/base/ftbase.c", src_dir),
        &format!("{}/src/base/ftbbox.c", src_dir),
        &format!("{}/src/base/ftglyph.c", src_dir),
        &format!("{}/src/base/ftbdf.c", src_dir),
        &format!("{}/src/base/ftbitmap.c", src_dir),
        &format!("{}/src/base/ftcid.c", src_dir),
        &format!("{}/src/base/ftfstype.c", src_dir),
        &format!("{}/src/base/ftgasp.c", src_dir),
        &format!("{}/src/base/ftgxval.c", src_dir),
        &format!("{}/src/base/ftmm.c", src_dir),
        &format!("{}/src/base/ftotval.c", src_dir),
        &format!("{}/src/base/ftpatent.c", src_dir),
        &format!("{}/src/base/ftpfr.c", src_dir),
        &format!("{}/src/base/ftstroke.c", src_dir),
        &format!("{}/src/base/ftsynth.c", src_dir),
        &format!("{}/src/base/fttype1.c", src_dir),
        &format!("{}/src/base/ftwinfnt.c", src_dir),
        &format!("{}/src/base/ftmac.c", src_dir),
        &format!("{}/src/bdf/bdf.c", src_dir),
        &format!("{}/src/cff/cff.c", src_dir),
        &format!("{}/src/cid/type1cid.c", src_dir),
        &format!("{}/src/pcf/pcf.c", src_dir),
        &format!("{}/src/pfr/pfr.c", src_dir),
        &format!("{}/src/sfnt/sfnt.c", src_dir),
        &format!("{}/src/truetype/truetype.c", src_dir),
        &format!("{}/src/type1/type1.c", src_dir),
        &format!("{}/src/type42/type42.c", src_dir),
        &format!("{}/src/winfonts/winfnt.c", src_dir),
        &format!("{}/src/smooth/smooth.c", src_dir),
        &format!("{}/src/raster/raster.c", src_dir),
        &format!("{}/src/sdf/sdf.c", src_dir),
        &format!("{}/src/autofit/autofit.c", src_dir),
        &format!("{}/src/cache/ftcache.c", src_dir),
        &format!("{}/src/gzip/ftgzip.c", src_dir),
        &format!("{}/src/lzw/ftlzw.c", src_dir),
        &format!("{}/src/bzip2/ftbzip2.c", src_dir),
        &format!("{}/src/gxvalid/gxvalid.c", src_dir),
        &format!("{}/src/otvalid/otvalid.c", src_dir),
        &format!("{}/src/psaux/psaux.c", src_dir),
        &format!("{}/src/pshinter/pshinter.c", src_dir),
        &format!("{}/src/psnames/psnames.c", src_dir),
        &format!("{}/src/svg/ftsvg.c", src_dir),
    ];

    cc::Build::new()
        .files(c_files)
        .include(&include_dir)
        .define("FT2_BUILD_LIBRARY", None)
        .compile("freetype");

    vec![include_dir]
}

#[cfg(all(feature = "lunasvg", feature = "use-submodules"))]
fn build_lunasvg_from_git() -> Vec<impl AsRef<std::path::Path>> {
    let src_dir = "third-party/lunasvg";
    let include_dir = format!("{}/include", src_dir);
    let plutovg_out_dir = format!("{}/3rdparty/plutovg", src_dir);

    println!("cargo:rerun-if-changed={}", src_dir);

    let lunasvg_source = [
        &format!("{}/source/lunasvg.cpp", src_dir),
        &format!("{}/source/element.cpp", src_dir),
        &format!("{}/source/property.cpp", src_dir),
        &format!("{}/source/parser.cpp", src_dir),
        &format!("{}/source/layoutcontext.cpp", src_dir),
        &format!("{}/source/canvas.cpp", src_dir),
        &format!("{}/source/clippathelement.cpp", src_dir),
        &format!("{}/source/defselement.cpp", src_dir),
        &format!("{}/source/gelement.cpp", src_dir),
        &format!("{}/source/geometryelement.cpp", src_dir),
        &format!("{}/source/graphicselement.cpp", src_dir),
        &format!("{}/source/maskelement.cpp", src_dir),
        &format!("{}/source/markerelement.cpp", src_dir),
        &format!("{}/source/paintelement.cpp", src_dir),
        &format!("{}/source/stopelement.cpp", src_dir),
        &format!("{}/source/styledelement.cpp", src_dir),
        &format!("{}/source/styleelement.cpp", src_dir),
        &format!("{}/source/svgelement.cpp", src_dir),
        &format!("{}/source/symbolelement.cpp", src_dir),
        &format!("{}/source/useelement.cpp", src_dir),
    ];

    let plutovg_source = [
        &format!("{}/plutovg.c", plutovg_out_dir),
        &format!("{}/plutovg-paint.c", plutovg_out_dir),
        &format!("{}/plutovg-geometry.c", plutovg_out_dir),
        &format!("{}/plutovg-blend.c", plutovg_out_dir),
        &format!("{}/plutovg-rle.c", plutovg_out_dir),
        &format!("{}/plutovg-dash.c", plutovg_out_dir),
        &format!("{}/plutovg-ft-raster.c", plutovg_out_dir),
        &format!("{}/plutovg-ft-stroker.c", plutovg_out_dir),
        &format!("{}/plutovg-ft-math.c", plutovg_out_dir),
    ];

    cc::Build::new()
        .files(lunasvg_source)
        .files(plutovg_source)
        .include(&include_dir)
        .include(&plutovg_out_dir)
        .cpp(true)
        .flag_if_supported("-std:c++17")
        .flag_if_supported("-std=c++17")
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
    build_freetype_from_git()
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
    build_lunasvg_from_git()
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
