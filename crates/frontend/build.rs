// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães <pedrenriquegg@hotmail.com>
//
// SPDX-License-Identifier: MIT

fn main() {
    #[cfg(target_os = "windows")]
    {
        let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        if target_arch != "wasm32" {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("../../images/sturdygb.ico");
            res.compile().expect("Failed to compile Windows resources");
        }
    }
}
