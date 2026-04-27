// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

mod visual_capture_common;

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use clap::Parser;
use serde::Deserialize;
use sturdygb_core::gb::ModelSelection;
use sturdygb_core::test_roms::{
    capture_visual_test, VisualCapture, VisualCaptureConfig, VisualCaptureOutcome,
    VisualStopCondition, DEFAULT_VISUAL_STEP_LIMIT,
};
use walkdir::WalkDir;

use crate::visual_capture_common::{sanitize_file_stem, save_captured_screen_png, DmgPalette};

#[derive(Parser, Debug)]
#[command(name = "capture_visual_tests")]
struct Cli {
    #[arg(long, value_name = "FILE")]
    manifest: Option<PathBuf>,
    #[arg(long, value_name = "DIR")]
    output_dir: Option<PathBuf>,
    #[arg(long = "case", value_name = "NAME")]
    case_filters: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct VisualCaptureManifest {
    #[serde(default)]
    cases: Vec<VisualCaptureCase>,
    #[serde(default)]
    suites: Vec<VisualCaptureSuite>,
}

#[derive(Clone, Debug, Deserialize)]
struct VisualCaptureCase {
    name: String,
    #[serde(default)]
    palette: DmgPalette,
    #[serde(default)]
    output_subdir: Option<String>,
    #[serde(flatten)]
    capture: VisualCaptureConfig,
}

#[derive(Clone, Debug, Deserialize)]
struct VisualCaptureSuite {
    name: String,
    directory: String,
    #[serde(default)]
    recursive: bool,
    #[serde(default)]
    palette: DmgPalette,
    #[serde(default)]
    output_subdir: Option<String>,
    #[serde(default)]
    include: Vec<String>,
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(default)]
    infer_model_from_name: bool,
    #[serde(default)]
    model_selection: ModelSelection,
    #[serde(default = "default_step_limit")]
    step_limit: usize,
    stop_condition: VisualStopCondition,
}

fn default_step_limit() -> usize {
    DEFAULT_VISUAL_STEP_LIMIT
}

fn default_manifest_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("roms")
        .join("visual-tests.toml")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("screenshots")
        .join("visual-tests")
}

fn case_is_selected(case_name: &str, filters: &[String]) -> bool {
    filters.is_empty()
        || filters
            .iter()
            .any(|filter| case_name.to_ascii_lowercase().contains(&filter.to_ascii_lowercase()))
}

fn relative_path_to_string(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn suite_entry_is_selected(relative_path: &str, suite: &VisualCaptureSuite) -> bool {
    let relative_path = relative_path.to_ascii_lowercase();
    let include_matches = suite.include.is_empty()
        || suite
            .include
            .iter()
            .any(|item| relative_path.contains(&item.to_ascii_lowercase()));
    let exclude_matches = suite
        .exclude
        .iter()
        .any(|item| relative_path.contains(&item.to_ascii_lowercase()));

    include_matches && !exclude_matches
}

fn infer_model_selection(relative_path: &str, fallback: ModelSelection) -> ModelSelection {
    let file_name = relative_path.to_ascii_lowercase();
    if file_name.contains("cgb") || file_name.ends_with("-c.gb") || file_name.ends_with("-c.gbc") {
        ModelSelection::Cgb
    } else {
        fallback
    }
}

fn expand_suite_cases(
    manifest_root: &Path,
    suite: &VisualCaptureSuite,
) -> Result<Vec<VisualCaptureCase>, String> {
    let suite_root = manifest_root.join(&suite.directory);
    if !suite_root.is_dir() {
        return Err(format!(
            "suite {} directory does not exist: {}",
            suite.name,
            suite_root.display()
        ));
    }

    let max_depth = if suite.recursive { usize::MAX } else { 1 };
    let mut cases = Vec::new();

    for entry in WalkDir::new(&suite_root)
        .min_depth(1)
        .max_depth(max_depth)
        .sort_by_file_name()
    {
        let entry = entry.map_err(|err| format!("failed to walk {}: {err}", suite_root.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }

        let extension = entry
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase());
        if !matches!(extension.as_deref(), Some("gb") | Some("gbc")) {
            continue;
        }

        let relative_to_suite = entry
            .path()
            .strip_prefix(&suite_root)
            .map_err(|err| format!("failed to normalize {}: {err}", entry.path().display()))?;
        let relative_to_suite = relative_path_to_string(relative_to_suite);
        if !suite_entry_is_selected(&relative_to_suite, suite) {
            continue;
        }

        let relative_to_manifest = entry
            .path()
            .strip_prefix(manifest_root)
            .map_err(|err| format!("failed to relativize {}: {err}", entry.path().display()))?;
        let rom_relative_path = relative_path_to_string(relative_to_manifest);
        let case_suffix = relative_to_suite
            .trim_end_matches(".gb")
            .trim_end_matches(".gbc")
            .replace('/', " :: ");
        cases.push(VisualCaptureCase {
            name: format!("{} :: {}", suite.name, case_suffix),
            palette: suite.palette,
            output_subdir: suite.output_subdir.clone(),
            capture: VisualCaptureConfig {
                rom_relative_path,
                model_selection: if suite.infer_model_from_name {
                    infer_model_selection(&relative_to_suite, suite.model_selection)
                } else {
                    suite.model_selection
                },
                step_limit: suite.step_limit,
                stop_condition: suite.stop_condition.clone(),
            },
        });
    }

    if cases.is_empty() {
        return Err(format!(
            "suite {} did not resolve any ROM files under {}",
            suite.name,
            suite_root.display()
        ));
    }

    Ok(cases)
}

fn collect_manifest_cases(
    manifest_root: &Path,
    manifest: &VisualCaptureManifest,
) -> Result<Vec<VisualCaptureCase>, String> {
    let mut cases = manifest.cases.clone();
    for suite in &manifest.suites {
        cases.extend(expand_suite_cases(manifest_root, suite)?);
    }
    Ok(cases)
}

fn write_capture_metadata(output_path: &Path, capture: &VisualCapture) -> Result<(), String> {
    let metadata = format!(
        "status = {}\nresult = {}\nrom = {}\nmodel = {}\nframes = {}\nsteps = {}\npc = {:#06X}\nserial = {:?}\n",
        capture.outcome.as_str(),
        capture.result_summary.as_deref().unwrap_or(""),
        capture.rom_relative_path,
        capture.model_selection.as_str(),
        capture.completed_frames,
        capture.executed_steps,
        capture.final_pc,
        capture.serial_output,
    );
    std::fs::write(output_path, metadata)
        .map_err(|err| format!("failed to write {}: {err}", output_path.display()))
}

#[derive(Debug)]
struct ReportRow {
    name: String,
    status: String,
    result: String,
    rom: String,
    model: String,
    frames: usize,
    steps: usize,
    pc: String,
    image_file_name: String,
    metadata_file_name: String,
    result_preview: String,
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn report_status_summary(rows: &[ReportRow], outcome: VisualCaptureOutcome) -> usize {
    rows.iter()
        .filter(|row| row.status == outcome.as_str())
        .count()
}

fn make_result_preview(result_summary: Option<&str>, serial_output: &str) -> String {
    let mut sections = Vec::new();
    if let Some(summary) = result_summary.filter(|summary| !summary.trim().is_empty()) {
        sections.push(summary.trim().to_string());
    }

    let trimmed = serial_output.trim();
    if !trimmed.is_empty() {
        const MAX_CHARS: usize = 240;
        let mut preview = String::with_capacity(trimmed.len().min(MAX_CHARS));
        for ch in trimmed.chars().take(MAX_CHARS) {
            preview.push(ch);
        }
        if trimmed.chars().count() > MAX_CHARS {
            preview.push_str("\n...");
        }
        sections.push(preview);
    }

    if sections.is_empty() {
        "(no serial output)".to_string()
    } else {
        sections.join("\n\n")
    }
}

fn write_html_report(output_path: &Path, rows: &[ReportRow]) -> Result<(), String> {
    let mut html = String::new();
    let stop_condition_met = report_status_summary(rows, VisualCaptureOutcome::StopConditionMet);
    let step_limit_reached = report_status_summary(rows, VisualCaptureOutcome::StepLimitReached);

    html.push_str(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>SturdyGB Test ROM Report</title>\n<style>\n",
    );
    html.push_str(
        "body { font-family: Segoe UI, sans-serif; margin: 24px; background: #f6f7f9; color: #1d2430; }\n",
    );
    html.push_str(
        "h1 { margin-bottom: 8px; }\n.summary { margin-bottom: 24px; }\n.summary strong { display: inline-block; min-width: 180px; }\n",
    );
    html.push_str(
        "table { width: 100%; border-collapse: collapse; background: #fff; box-shadow: 0 8px 30px rgba(15, 23, 42, 0.08); }\n",
    );
    html.push_str(
        "th, td { padding: 12px; border-bottom: 1px solid #dde3ea; text-align: left; vertical-align: top; }\n",
    );
    html.push_str(
        ".status-stop-condition-met { color: #0a6c39; font-weight: 700; }\n.status-step-limit-reached { color: #8a5a00; font-weight: 700; }\n",
    );
    html.push_str(
        "img { width: 240px; height: auto; image-rendering: pixelated; border: 1px solid #ccd3db; background: #fff; }\n",
    );
    html.push_str(
        "pre { margin: 0; max-width: 340px; white-space: pre-wrap; font-family: Consolas, monospace; font-size: 12px; }\n",
    );
    html.push_str(
        ".no-frame { display: inline-block; min-width: 220px; padding: 16px; border: 1px dashed #ccd3db; background: #f8fafc; color: #516071; font-size: 13px; }\n",
    );
    html.push_str("a { color: #005fb8; text-decoration: none; }\na:hover { text-decoration: underline; }\n</style>\n</head>\n<body>\n");
    html.push_str("<h1>SturdyGB Test ROM Report</h1>\n<div class=\"summary\">\n");
    let _ = writeln!(
        html,
        "<div><strong>Total cases</strong> {}</div>",
        rows.len()
    );
    let _ = writeln!(
        html,
        "<div><strong>Stop condition met</strong> {}</div>",
        stop_condition_met
    );
    let _ = writeln!(
        html,
        "<div><strong>Step limit reached</strong> {}</div>",
        step_limit_reached
    );
    html.push_str("</div>\n<table>\n<thead><tr><th>Case</th><th>Status</th><th>Result</th><th>ROM</th><th>Model</th><th>Frames</th><th>Steps</th><th>PC</th><th>Output</th><th>Screenshot</th><th>Metadata</th></tr></thead>\n<tbody>\n");

    for row in rows {
        let screenshot_cell = if row.frames > 0 {
            format!(
                "<a href=\"{}\"><img src=\"{}\" alt=\"Screenshot for {}\"></a>",
                escape_html(&row.image_file_name),
                escape_html(&row.image_file_name),
                escape_html(&row.name),
            )
        } else {
            format!(
                "<div class=\"no-frame\">No video frame produced yet.<br><a href=\"{}\">Open raw PNG</a></div>",
                escape_html(&row.image_file_name),
            )
        };
        let _ = writeln!(
            html,
            "<tr><td>{}</td><td class=\"status-{}\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><pre>{}</pre></td><td>{}</td><td><a href=\"{}\">metadata</a></td></tr>",
            escape_html(&row.name),
            escape_html(&row.status),
            escape_html(&row.status),
            escape_html(&row.result),
            escape_html(&row.rom),
            escape_html(&row.model),
            row.frames,
            row.steps,
            escape_html(&row.pc),
            escape_html(&row.result_preview),
            screenshot_cell,
            escape_html(&row.metadata_file_name),
        );
    }

    html.push_str("</tbody>\n</table>\n</body>\n</html>\n");

    std::fs::write(output_path, html)
        .map_err(|err| format!("failed to write {}: {err}", output_path.display()))
}

fn main() {
    let cli = Cli::parse();
    let manifest_path = cli.manifest.unwrap_or_else(default_manifest_path);
    let output_dir = cli.output_dir.unwrap_or_else(default_output_dir);
    let manifest_text = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", manifest_path.display()));
    let manifest: VisualCaptureManifest = toml::from_str(&manifest_text)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", manifest_path.display()));
    let manifest_root = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let cases = collect_manifest_cases(manifest_root, &manifest)
        .unwrap_or_else(|err| panic!("failed to expand manifest {}: {err}", manifest_path.display()));

    std::fs::create_dir_all(&output_dir)
        .unwrap_or_else(|err| panic!("failed to create {}: {err}", output_dir.display()));

    let mut summary = String::from("name\tstatus\tresult\trom\tmodel\tframes\tsteps\tpc\timage\tmetadata\n");
    let mut report_rows = Vec::new();

    for case in cases
        .iter()
        .filter(|case| case_is_selected(&case.name, &cli.case_filters))
    {
        let stem = sanitize_file_stem(&case.name);
        let image_relative_path = case
            .output_subdir
            .as_deref()
            .map(PathBuf::from)
            .unwrap_or_default()
            .join(format!("{stem}.png"));
        let metadata_relative_path = case
            .output_subdir
            .as_deref()
            .map(PathBuf::from)
            .unwrap_or_default()
            .join(format!("{stem}.txt"));
        let image_path = output_dir.join(&image_relative_path);
        let metadata_path = output_dir.join(&metadata_relative_path);

        let capture = capture_visual_test(&case.capture);
        save_captured_screen_png(&capture.screen, case.palette, &image_path)
            .unwrap_or_else(|err| panic!("failed to save {}: {err}", image_path.display()));
        write_capture_metadata(&metadata_path, &capture)
            .unwrap_or_else(|err| panic!("failed to write {}: {err}", metadata_path.display()));
        summary.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:#06X}\t{}\t{}\n",
            case.name,
            capture.outcome.as_str(),
            capture.result_summary.as_deref().unwrap_or(""),
            capture.rom_relative_path,
            capture.model_selection.as_str(),
            capture.completed_frames,
            capture.executed_steps,
            capture.final_pc,
            relative_path_to_string(&image_relative_path),
            relative_path_to_string(&metadata_relative_path),
        ));
        report_rows.push(ReportRow {
            name: case.name.clone(),
            status: capture.outcome.as_str().to_string(),
            result: capture.result_summary.clone().unwrap_or_default(),
            rom: capture.rom_relative_path.clone(),
            model: capture.model_selection.as_str().to_string(),
            frames: capture.completed_frames,
            steps: capture.executed_steps,
            pc: format!("{:#06X}", capture.final_pc),
            image_file_name: relative_path_to_string(&image_relative_path),
            metadata_file_name: relative_path_to_string(&metadata_relative_path),
            result_preview: make_result_preview(
                capture.result_summary.as_deref(),
                &capture.serial_output,
            ),
        });
        println!(
            "Captured {} -> {} ({})",
            case.name,
            image_path.display(),
            capture.outcome.as_str()
        );
    }

    if report_rows.is_empty() {
        panic!("no manifest cases matched the provided filters");
    }

    let summary_path = output_dir.join("summary.tsv");
    std::fs::write(&summary_path, summary)
        .unwrap_or_else(|err| panic!("failed to write {}: {err}", summary_path.display()));

    let report_path = output_dir.join("report.html");
    write_html_report(&report_path, &report_rows)
        .unwrap_or_else(|err| panic!("failed to write {}: {err}", report_path.display()));

    println!(
        "Finished capture run: {} cases. Summary: {} Report: {}",
        report_rows.len(),
        summary_path.display(),
        report_path.display()
    );
}