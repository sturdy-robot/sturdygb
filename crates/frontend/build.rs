// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães <pedrenriquegg@hotmail.com>
//
// SPDX-License-Identifier: MIT

fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../images/sturdygb.ico");
        res.compile().expect("Failed to compile Windows resources");
    }
}
