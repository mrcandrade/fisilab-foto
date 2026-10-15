//! Windows only: embed the app icon and version info (VERSIONINFO) into `fisilab-foto-cli.exe`
//! (Suíte FisiLab; the package keeps its upstream name, `photocraft-cli`).
//!
//! On every other target this does nothing, even when `PHOTOCRAFT_REQUIRE_WINRES` is set. A
//! missing resource compiler is a warning, so a cross-compile from macOS or Linux still links,
//! unless `PHOTOCRAFT_REQUIRE_WINRES=1` (set by the release packaging script) turns it into a
//! build error.

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/fisilab/fisilab-foto.ico");
    println!("cargo:rerun-if-env-changed=PHOTOCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    let mut res = winresource::WindowsResource::new();
    // FileVersion / ProductVersion strings and the numeric FILEVERSION / PRODUCTVERSION are left
    // to `WindowsResource::new`, which reads the Cargo package version. A pre-release such as
    // `1.2.3-rc.1` stays in the strings; the numeric fields are `1.2.3.0`.
    res.set_icon("../../assets/fisilab/fisilab-foto.ico")
        .set("ProductName", "Foto")
        .set("FileDescription", "Foto — Editor de imagens, linha de comando (Suíte FisiLab)")
        .set("CompanyName", "FisiLab — LARFIC/UFRGS")
        .set("LegalCopyright", "Baseado no PhotoCraft (© ArtCraft Team e contribuidores). Modificações © 2026 FisiLab — LARFIC/UFRGS. MIT OR Apache-2.0.")
        .set("OriginalFilename", "fisilab-foto-cli.exe")
        .set("InternalName", "fisilab-foto-cli");
    if let Err(err) = res.compile() {
        if std::env::var_os("PHOTOCRAFT_REQUIRE_WINRES").is_some() {
            return Err(std::io::Error::other(format!(
                "embedding Windows resources into fisilab-foto-cli.exe failed: {err}. \
                 Install the Windows SDK resource compiler (rc.exe) or mingw-w64 windres and rebuild."
            )));
        }
        println!("cargo:warning=fisilab-foto-cli.exe built without icon/version resources: {err}");
    }
    Ok(())
}
