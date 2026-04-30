#[cfg(not(target_arch = "wasm32"))]
use super::super::config::SortMethod;
use super::super::EmuApp;
use eframe::egui;

impl EmuApp {
    pub(super) fn show_game_library_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        self.show_native_library_panel(ui, frame);

        #[cfg(target_arch = "wasm32")]
        self.show_wasm_picker_panel(ui, frame);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_native_library_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        if self.config.rom_directories.is_empty()
            && self.catalog.game_list.is_empty()
            && !self.catalog.loading_directory
        {
            self.show_empty_library_panel(ui, frame);
            return;
        }

        self.show_directory_chips(ui);
        ui.separator();

        if self.catalog.loading_directory {
            self.show_loading_library_panel(ui);
            return;
        }

        self.show_game_table(ui, frame);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_empty_library_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 2.0 - 30.0);
                ui.heading("No games found.");
                ui.add_space(8.0);
                if ui.button("📁 Open ROM...").clicked() {
                    self.open_rom_from_picker_with_storage(frame.storage());
                }
                if ui.button("📁 Add ROM directory...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.load_directory(path);
                    }
                }
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_directory_chips(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label("Directories:");
            let mut to_remove = None;
            for (index, dir) in self.config.rom_directories.iter().enumerate() {
                let dir_name = dir.file_name().unwrap_or_default().to_string_lossy();
                let response = ui.button(format!("{} ❌", dir_name));
                if response.clicked() {
                    to_remove = Some(index);
                }
            }
            if let Some(index) = to_remove {
                self.config.rom_directories.remove(index);
                self.reload_all_directories();
            }
            if ui.button("+ Add").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.load_directory(path);
                }
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_loading_library_panel(&self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.add_space(ui.available_height() / 2.0 - 30.0);
            ui.vertical_centered(|ui| {
                ui.heading(format!(
                    "Loading Games... ({})",
                    self.catalog.game_list.len()
                ));
                ui.add(egui::Spinner::new().size(32.0));
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_game_table(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let mut to_load = None;

        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.catalog.search_query);

            ui.separator();

            ui.label("Sort by:");
            egui::ComboBox::from_id_salt("sort_by")
                .selected_text(format!("{:?}", self.catalog.sort_by))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.catalog.sort_by,
                        SortMethod::Filename,
                        "Filename",
                    );
                    ui.selectable_value(&mut self.catalog.sort_by, SortMethod::Title, "Title");
                    ui.selectable_value(
                        &mut self.catalog.sort_by,
                        SortMethod::Company,
                        "Company",
                    );
                });

            if ui
                .button(if self.catalog.sort_ascending { "⬆" } else { "⬇" })
                .clicked()
            {
                self.catalog.sort_ascending = !self.catalog.sort_ascending;
            }
        });
        ui.add_space(4.0);

        let query = self.catalog.search_query.to_lowercase();
        let mut filtered_games: Vec<_> = self
            .catalog
            .game_list
            .iter()
            .filter(|game| {
                query.is_empty()
                    || game.filename.to_lowercase().contains(&query)
                    || game.title.to_lowercase().contains(&query)
                    || game.company.to_lowercase().contains(&query)
            })
            .collect();

        filtered_games.sort_by(|left, right| {
            let cmp = match self.catalog.sort_by {
                SortMethod::Filename => left.filename.cmp(&right.filename),
                SortMethod::Title => left.title.cmp(&right.title),
                SortMethod::Company => left.company.cmp(&right.company),
            };
            if self.catalog.sort_ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });

        let row_height = 20.0;

        use egui_extras::{Column, TableBuilder};
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(
                Column::auto_with_initial_suggestion(300.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(
                Column::auto_with_initial_suggestion(150.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::remainder())
            .min_scrolled_height(0.0);

        table
            .header(row_height, |mut header| {
                header.col(|ui| {
                    ui.strong("Filename");
                });
                header.col(|ui| {
                    ui.strong("Title");
                });
                header.col(|ui| {
                    ui.strong("Company");
                });
            })
            .body(|body| {
                body.rows(row_height, filtered_games.len(), |mut row| {
                    let entry = filtered_games[row.index()];
                    row.col(|ui| {
                        if ui.selectable_label(false, &entry.filename).double_clicked() {
                            to_load = Some(entry.path.clone());
                        }
                    });
                    row.col(|ui| {
                        ui.label(&entry.title);
                    });
                    row.col(|ui| {
                        ui.label(&entry.company);
                    });
                });
            });

        if let Some(path) = to_load {
            self.load_rom_file(path.to_str().unwrap(), frame.storage());
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn show_wasm_picker_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 2.0 - 30.0);
                ui.heading(super::super::APP_NAME);
                ui.heading("Select a ROM file");
                ui.add_space(8.0);
                if ui.button("📁 Open ROM...").clicked() {
                    self.open_rom_from_picker_with_storage(frame.storage());
                }
            });
        });
    }
}