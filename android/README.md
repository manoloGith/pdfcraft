# Android shims (DexPdfCraft fork)

PdfCraft's egui interface uses two desktop crates with no Android backend: `rfd` (file and message
dialogs) and `arboard` (the system clipboard). On Android, `crates/ui-egui/Cargo.toml` takes these
two crates under the same names instead, so the interface code is unchanged:

- `rfd-shim`: `AsyncFileDialog`, `FileHandle` and the message dialogs, with the picker answered by
  a [`Backend`](rfd-shim/src/lib.rs) the Android app installs (the Storage Access Framework, over
  JNI). Without a backend every picker is cancelled.
- `arboard-shim`: `Clipboard` with text, HTML, image and file-list, answered by a backend the app
  installs (Android's `ClipboardManager`). Without one, an in-process clipboard.

They exist only in the DexPdfCraft fork (branch `dexpdfcraft/android`) and are not built on other
platforms.
