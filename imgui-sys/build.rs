#![allow(dead_code)]

const DEFINES: &[(&str, Option<&str>)] = &[
    // Rust `char` is a unicode scalar value, e.g. 32 bits.
    ("IMGUI_USE_WCHAR32", None),
    // Disabled due to linking issues
    ("CIMGUI_NO_EXPORT", None),
    ("IMGUI_DISABLE_WIN32_FUNCTIONS", None),
    ("IMGUI_DISABLE_OSX_FUNCTIONS", None),
];

#[cfg(feature = "use-git")]
fn clone_repo_version(url: &str, out_dir: &str, ver: &str) {
    if !std::path::Path::new(out_dir).exists() {
        let repo = match git2::Repository::clone_recurse(url, out_dir) {
            Ok(repo) => repo,
            Err(err) => panic!("cannot find lunasvg: {}", err),
        };
        
        let (object, reference) = repo.revparse_ext(ver).expect("Object not found");
    
        repo.checkout_tree(&object, None).expect("Failed to checkout");
    
        let _ = match reference {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(object.id()),
        };
    }
}

#[cfg(all(feature = "freetype", feature = "use-git"))]
fn build_freetype_from_git() -> Vec<impl AsRef<std::path::Path>> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = format!("{}/freetype", out_dir);
    let include_dir = format!("{}/include", out_dir);

    clone_repo_version("https://github.com/freetype/freetype", &out_dir, "VER-2-13-3");

    println!("cargo:rerun-if-changed={}", out_dir);

    let c_files = [
        &format!("{}/src/base/ftsystem.c", out_dir),
        &format!("{}/src/base/ftinit.c", out_dir),
        &format!("{}/src/base/ftdebug.c", out_dir),
        &format!("{}/src/base/ftbase.c", out_dir),
        &format!("{}/src/base/ftbbox.c", out_dir),
        &format!("{}/src/base/ftglyph.c", out_dir),
        &format!("{}/src/base/ftbdf.c", out_dir),
        &format!("{}/src/base/ftbitmap.c", out_dir),
        &format!("{}/src/base/ftcid.c", out_dir),
        &format!("{}/src/base/ftfstype.c", out_dir),
        &format!("{}/src/base/ftgasp.c", out_dir),
        &format!("{}/src/base/ftgxval.c", out_dir),
        &format!("{}/src/base/ftmm.c", out_dir),
        &format!("{}/src/base/ftotval.c", out_dir),
        &format!("{}/src/base/ftpatent.c", out_dir),
        &format!("{}/src/base/ftpfr.c", out_dir),
        &format!("{}/src/base/ftstroke.c", out_dir),
        &format!("{}/src/base/ftsynth.c", out_dir),
        &format!("{}/src/base/fttype1.c", out_dir),
        &format!("{}/src/base/ftwinfnt.c", out_dir),
        &format!("{}/src/base/ftmac.c", out_dir),
        &format!("{}/src/bdf/bdf.c", out_dir),
        &format!("{}/src/cff/cff.c", out_dir),
        &format!("{}/src/cid/type1cid.c", out_dir),
        &format!("{}/src/pcf/pcf.c", out_dir),
        &format!("{}/src/pfr/pfr.c", out_dir),
        &format!("{}/src/sfnt/sfnt.c", out_dir),
        &format!("{}/src/truetype/truetype.c", out_dir),
        &format!("{}/src/type1/type1.c", out_dir),
        &format!("{}/src/type42/type42.c", out_dir),
        &format!("{}/src/winfonts/winfnt.c", out_dir),
        &format!("{}/src/smooth/smooth.c", out_dir),
        &format!("{}/src/raster/raster.c", out_dir),
        &format!("{}/src/sdf/sdf.c", out_dir),
        &format!("{}/src/autofit/autofit.c", out_dir),
        &format!("{}/src/cache/ftcache.c", out_dir),
        &format!("{}/src/gzip/ftgzip.c", out_dir),
        &format!("{}/src/lzw/ftlzw.c", out_dir),
        &format!("{}/src/bzip2/ftbzip2.c", out_dir),
        &format!("{}/src/gxvalid/gxvalid.c", out_dir),
        &format!("{}/src/otvalid/otvalid.c", out_dir),
        &format!("{}/src/psaux/psaux.c", out_dir),
        &format!("{}/src/pshinter/pshinter.c", out_dir),
        &format!("{}/src/psnames/psnames.c", out_dir),
        &format!("{}/src/svg/ftsvg.c", out_dir),
    ];

    cc::Build::new()
        .files(c_files)
        .include(&include_dir)
        .define("FT2_BUILD_LIBRARY", None)
        .compile("freetype");

    vec![include_dir]
}

#[cfg(all(feature = "lunasvg", feature = "use-git"))]
fn build_lunasvg_from_git() -> Vec<impl AsRef<std::path::Path>> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = format!("{}/lunasvg", out_dir);
    let include_dir = format!("{}/include", out_dir);
    let plutovg_out_dir = format!("{}/3rdparty/plutovg", out_dir);

    clone_repo_version("https://github.com/sammycage/lunasvg", &out_dir, "v2.4.1");

    println!("cargo:rerun-if-changed={}", out_dir);

    let lunasvg_source = [
        &format!("{}/source/lunasvg.cpp", out_dir),
        &format!("{}/source/element.cpp", out_dir),
        &format!("{}/source/property.cpp", out_dir),
        &format!("{}/source/parser.cpp", out_dir),
        &format!("{}/source/layoutcontext.cpp", out_dir),
        &format!("{}/source/canvas.cpp", out_dir),
        &format!("{}/source/clippathelement.cpp", out_dir),
        &format!("{}/source/defselement.cpp", out_dir),
        &format!("{}/source/gelement.cpp", out_dir),
        &format!("{}/source/geometryelement.cpp", out_dir),
        &format!("{}/source/graphicselement.cpp", out_dir),
        &format!("{}/source/maskelement.cpp", out_dir),
        &format!("{}/source/markerelement.cpp", out_dir),
        &format!("{}/source/paintelement.cpp", out_dir),
        &format!("{}/source/stopelement.cpp", out_dir),
        &format!("{}/source/styledelement.cpp", out_dir),
        &format!("{}/source/styleelement.cpp", out_dir),
        &format!("{}/source/svgelement.cpp", out_dir),
        &format!("{}/source/symbolelement.cpp", out_dir),
        &format!("{}/source/useelement.cpp", out_dir),
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
    #[cfg(not(any(feature = "use-vcpkg", feature = "use-git")))]
    match pkg_config::Config::new().find("freetype2") {
        Ok(freetype) => freetype.include_paths,
        Err(err) => panic!("cannot find freetype: {}", err),
    }
    #[cfg(feature = "use-vcpkg")]
    match vcpkg::find_package("freetype") {
        Ok(freetype) => freetype.include_paths,
        Err(err) => panic!("cannot find freetype: {}", err),
    }
    #[cfg(feature = "use-git")]
    build_freetype_from_git()
}

#[cfg(feature = "lunasvg")]
fn find_lunasvg() -> Vec<impl AsRef<std::path::Path>> {
    #[cfg(not(any(feature = "use-vcpkg", feature="use-git")))]
    match pkg_config::Config::new().find("lunasvg") {
        Ok(lunasvg) => lunasvg.include_paths,
        Err(err) => panic!("cannot find lunasvg: {}", err),
    }
    #[cfg(feature = "use-vcpkg")]
    match vcpkg::find_package("freetype") {
        Ok(freetype) => freetype.include_paths,
        Err(err) => panic!("cannot find lunasvg: {}", err),
    }
    #[cfg(feature = "use-git")]
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
