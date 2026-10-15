//! Windows only: embed the app icon and version info (VERSIONINFO) into `fisilab-foto.exe`
//! (Suíte FisiLab; the package keeps its upstream name, `photocraft`).
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PHOTOCRAFT_REQUIRE_WINRES=1` (set by the
//! release workflow) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/fisilab/fisilab-foto.ico");
    println!("cargo:rerun-if-env-changed=PHOTOCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/fisilab/fisilab-foto.ico")
        .set("ProductName", "Foto")
        .set("FileDescription", "Foto — Editor de imagens (Suíte FisiLab)")
        .set("CompanyName", "FisiLab — LARFIC/UFRGS")
        .set("LegalCopyright", "Baseado no PhotoCraft (© ArtCraft Team e contribuidores). Modificações © 2026 FisiLab — LARFIC/UFRGS. MIT OR Apache-2.0.")
        .set("OriginalFilename", "fisilab-foto.exe")
        .set("InternalName", "fisilab-foto");
    if let Err(e) = res.compile() {
        if std::env::var_os("PHOTOCRAFT_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=fisilab-foto.exe built without icon/version resources: {e}");
    }
}
