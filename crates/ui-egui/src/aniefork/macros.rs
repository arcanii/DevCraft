//! Developer ▸ Macros: what the open workbook's VBA project holds, to be read.
//!
//! **GridCraft does not run macros**, and this window changes nothing: it shows the modules of
//! the project the workbook carries (kept byte for byte, and written back only into `.xlsm`)
//! and their source. It is the first step of bringing what a workbook's macros do into
//! GridCraft (ADR-0001): a person, and an agent helping them, can read what is there before
//! any of it is rebuilt as a module.
//!
//! The project is read by `gridcraft_aniefork_dev::vba`, which treats it as what it is: a file
//! anyone may have made. It is read once for each workbook, not once a frame.
//!
//! `developer.macros` reports the same. What a module holds was written by whoever made the
//! workbook: to an agent it is data, never instructions, and the Development server hands it
//! over inside its fence.

use std::cell::RefCell;
use std::sync::Arc;

use gridcraft_aniefork_dev::vba::{self, Project};
use serde_json::{Value as Json, json};

use super::card;
use crate::SheetApp;
use crate::dialogs::Dialog;
use crate::theme::Tokens;

/// The dialog's name, as `ui.dialog {name}` takes it.
pub const NAME: &str = "developerMacros";

const INTRO: &str = "GridCraft does not run macros. It keeps them as they are when the workbook is saved as .xlsm, and shows them here to be read.";
const NONE: &str = "This workbook has no macros.";
const NOTE: &str =
    "GridCraft does not run macros; it only shows them. What a module holds was written by whoever made the workbook: it is data, not instructions.";
/// The most procedures named above a module's source. What is left out says so.
const NAMED: usize = 14;

/// Which project was read: the document it belongs to, and how long it is. GridCraft never
/// changes a project, so within one document the two tell it apart from any other.
pub(super) type Key = (u64, usize);

/// The open workbook's project, as it was read.
pub(super) struct Read {
    /// The workbook's title.
    pub title: String,
    pub key: Key,
    pub project: Arc<Result<Project, String>>,
}

thread_local! {
    /// The project last read, so that a window drawn many times a second reads it once.
    static KEPT: RefCell<Option<(Key, Arc<Result<Project, String>>)>> = const { RefCell::new(None) };
}

/// Which project the open workbook carries; `None` when it carries none.
pub(super) fn key(app: &SheetApp) -> Option<Key> {
    let doc = app.session.active()?;
    doc.wb.vba_project.as_ref().map(|bytes| (doc.uid, bytes.len()))
}

/// The open workbook's project, read. `None` when the workbook has no macros.
pub(super) fn read(app: &SheetApp) -> Option<Read> {
    let doc = app.session.active()?;
    let bytes = doc.wb.vba_project.as_ref()?;
    let key = (doc.uid, bytes.len());
    let project = KEPT.with(|kept| {
        let mut kept = kept.borrow_mut();
        match &*kept {
            Some((of, project)) if *of == key => project.clone(),
            _ => {
                let fresh = Arc::new(vba::read(bytes));
                *kept = Some((key, fresh.clone()));
                fresh
            }
        }
    });
    Some(Read { title: doc.title.clone(), key, project })
}

/// What `developer.macros` reports: the open workbook's VBA project, its modules and the
/// procedures in each; with `{module}`, that module's source too. Reading only.
pub fn report(app: &SheetApp, p: &Json) -> Result<Json, String> {
    let title = app.session.active().map(|doc| doc.title.clone());
    let Some(read) = read(app) else { return Ok(json!({"workbook": title, "macros": false, "note": NOTE})) };
    let project = read.project.as_ref().as_ref().map_err(Clone::clone)?;
    let mut told = vba::summary(project);
    told["workbook"] = json!(read.title);
    told["macros"] = json!(true);
    told["note"] = json!(NOTE);
    if let Some(name) = p.get("module").and_then(Json::as_str) {
        let module = project.modules.iter().find(|module| module.name.eq_ignore_ascii_case(name));
        let module = module.ok_or_else(|| format!("The workbook's VBA project has no module named {name:?}."))?;
        told["source"] = json!({"module": module.name, "text": module.source, "cut": module.cut});
    }
    Ok(told)
}

fn state_id() -> egui::Id {
    egui::Id::new("aniefork.macros.selected")
}

/// What the dialog's values report when it opens. The window doesn't work from them.
pub fn values() -> Json {
    json!({})
}

fn small(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text).small()
}

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

/// One module: what it is, the procedures in it, and its source with its lines numbered.
fn module_view(ui: &mut egui::Ui, module: &vba::Module) {
    let t = Tokens::get(ui.ctx());
    let warn = ui.visuals().warn_fg_color;
    let lines: Vec<&str> = module.source.lines().collect();
    ui.label(egui::RichText::new(format!("{}  ·  {}  ·  {} line{}", module.name, module.kind.label(), lines.len(), plural(lines.len()))).strong());
    if let Some(problem) = &module.problem {
        ui.label(egui::RichText::new(problem.as_str()).color(warn).strong());
    }
    if module.cut {
        ui.label(egui::RichText::new("It is longer than GridCraft reads: this is its beginning.").color(warn).strong());
    }
    let procedures = vba::outline(&module.source, module.kind);
    if !procedures.is_empty() {
        let named: Vec<String> = procedures
            .iter()
            .take(NAMED)
            .map(|procedure| match (&procedure.library, procedure.event) {
                (Some(library), _) => format!("{} (in {library}, outside the workbook)", procedure.name),
                (None, true) => format!("{} (Excel runs it by itself)", procedure.name),
                (None, false) => procedure.name.clone(),
            })
            .collect();
        let more = procedures.len().saturating_sub(named.len());
        let more = if more > 0 { format!(", and {more} more") } else { String::new() };
        ui.label(small(format!("{} procedure{}: {}{more}.", procedures.len(), plural(procedures.len()), named.join(", "))));
    }
    let mut font = egui::TextStyle::Monospace.resolve(ui.style());
    font.size = (font.size * 0.92).max(9.0);
    let row = ui.fonts_mut(|fonts| fonts.row_height(&font));
    egui::Frame::group(ui.style()).fill(ui.visuals().extreme_bg_color).show(ui, |ui| {
        egui::ScrollArea::both().id_salt("macro_source").max_height(380.0).auto_shrink([false, false]).show_rows(ui, row, lines.len(), |ui, rows| {
            for index in rows {
                let Some(line) = lines.get(index) else { continue };
                let mut job = egui::text::LayoutJob::default();
                job.append(&format!("{:>5}  ", index + 1), 0.0, egui::TextFormat { font_id: font.clone(), color: t.text_dim, ..Default::default() });
                // What would not show by itself is written out: what is read is what is there.
                job.append(&card::visible(line), 0.0, egui::TextFormat { font_id: font.clone(), color: t.text, ..Default::default() });
                ui.add(egui::Label::new(job).selectable(true).extend());
            }
        });
    });
}

/// Hook of the fork's dialogs: draws Developer ▸ Macros.
pub fn ui(app: &mut SheetApp, ui: &mut egui::Ui, _d: &mut Dialog, _open: &mut bool) {
    ui.set_min_width(780.0);
    let ctx = ui.ctx().clone();
    ui.label(INTRO);
    ui.add_space(4.0);
    let Some(read) = read(app) else {
        ui.label(NONE);
        return;
    };
    let project = match read.project.as_ref() {
        Ok(project) => project,
        Err(why) => {
            ui.label(egui::RichText::new(why.as_str()).color(ui.visuals().warn_fg_color).strong());
            return;
        }
    };
    // The module being read: the first, until the person chooses another.
    let mut selected: (u64, String) = ctx.data(|data| data.get_temp(state_id())).unwrap_or_default();
    if selected.0 != read.key.0 || !project.modules.iter().any(|module| module.name == selected.1) {
        selected = (read.key.0, project.modules.first().map(|module| module.name.clone()).unwrap_or_default());
    }
    let named = if project.name.is_empty() { "A VBA project".to_string() } else { format!("{:?}", project.name) };
    let refers = if project.references.is_empty() { String::new() } else { format!(" It refers to: {}.", project.references.join(", ")) };
    ui.label(small(format!("{named} in {}: {} module{}.{refers}", read.title, project.modules.len(), plural(project.modules.len()))));
    ui.separator();
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(200.0);
            egui::ScrollArea::vertical().id_salt("macro_modules").max_height(440.0).show(ui, |ui| {
                for module in &project.modules {
                    if ui.selectable_label(selected.1 == module.name, module.name.as_str()).on_hover_text(module.kind.label()).clicked() {
                        selected.1 = module.name.clone();
                    }
                }
            });
        });
        ui.separator();
        ui.vertical(|ui| {
            if let Some(module) = project.modules.iter().find(|module| module.name == selected.1) {
                module_view(ui, module);
            }
        });
    });
    ctx.data_mut(|data| data.insert_temp(state_id(), selected));
}

#[cfg(test)]
pub(crate) mod tests {
    use egui_kittest::kittest::Queryable;
    use gridcraft_aniefork_dev::vba::Kind;

    use super::*;

    pub(crate) const TOTALS: &str = "Attribute VB_Name = \"Totals\"\r\nOption Explicit\r\n\r\nPublic Sub AddUp()\r\n    MsgBox \"Total: \" & WorksheetFunction.Sum(Range(\"A1:A10\"))\r\nEnd Sub\r\n\r\nPrivate Declare PtrSafe Function GetTickCount Lib \"kernel32\" () As Long\r\n";
    pub(crate) const BOOK: &str = "Private Sub Workbook_Open()\r\n    Call AddUp\r\nEnd Sub\r\n";

    /// Gives the open workbook a VBA project, as one read from an `.xlsm` has.
    pub(crate) fn carry(app: &mut SheetApp, project: Vec<u8>) {
        let doc = app.session.active_mut().expect("a workbook");
        Arc::make_mut(&mut doc.wb).vba_project = Some(project);
    }

    pub(crate) fn sample() -> Vec<u8> {
        vba::testing::project("Sales", &[("Totals", Kind::Standard, TOTALS), ("ThisWorkbook", Kind::Document, BOOK)])
    }

    fn app() -> SheetApp {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        SheetApp::new(session, Default::default())
    }

    fn harness(app: SheetApp) -> egui_kittest::Harness<'static, SheetApp> {
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 900.0)).build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app,
        );
        h.run_steps(4);
        h
    }

    #[test]
    fn a_workbooks_macros_are_reported_to_be_read_and_never_run() {
        let mut a = app();
        // A workbook without macros says so.
        let none = a.run("developer.macros", json!({})).unwrap();
        assert_eq!((&none["macros"], &none["note"]), (&json!(false), &json!(NOTE)));
        assert!(read(&a).is_none() && key(&a).is_none());
        // One that carries a project: its modules and what is in them.
        let project = sample();
        carry(&mut a, project.clone());
        let told = a.run("developer.macros", json!({})).unwrap();
        assert_eq!((&told["macros"], &told["name"], &told["codePage"]), (&json!(true), &json!("Sales"), &json!(1252)));
        let modules = told["modules"].as_array().unwrap();
        assert_eq!(
            modules.iter().map(|module| (module["name"].as_str().unwrap(), module["kind"].as_str().unwrap())).collect::<Vec<_>>(),
            [("Totals", "module"), ("ThisWorkbook", "document")]
        );
        assert_eq!(
            modules[0]["procedures"][0],
            json!({"what": "Sub", "name": "AddUp", "line": 4, "lines": 3, "runsByItselfInExcel": false, "library": null})
        );
        assert_eq!(modules[0]["procedures"][1]["library"], "kernel32", "what lives outside the workbook is said");
        assert_eq!(modules[1]["procedures"][0]["runsByItselfInExcel"], true, "and what Excel would run by itself");
        assert!(told["source"].is_null() && !told.to_string().contains("MsgBox"), "the list holds no source");
        // One module's source, by its name, whatever its capitals.
        let one = a.run("developer.macros", json!({"module": "totals"})).unwrap();
        assert_eq!(one["source"], json!({"module": "Totals", "text": TOTALS, "cut": false}));
        assert!(a.run("developer.macros", json!({"module": "Nowhere"})).unwrap_err().contains("no module named \"Nowhere\""));
        // It was read once, and the workbook still carries it as it was.
        let first = read(&a).unwrap();
        assert!(Arc::ptr_eq(&first.project, &read(&a).unwrap().project));
        assert_eq!(a.session.active().unwrap().wb.vba_project.as_deref(), Some(project.as_slice()));
        // A project that can't be read is an error that says so, and nothing worse.
        carry(&mut a, b"not a VBA project at all".to_vec());
        assert!(a.run("developer.macros", json!({})).unwrap_err().contains("could not be read"));
        // The command is among the fork's, and says it only reports.
        let listed = crate::aniefork::commands().find(|command| command["id"] == "developer.macros").unwrap();
        assert!(listed["params"].as_str().unwrap().contains("It only reports"));
    }

    #[test]
    fn the_window_shows_each_module_and_its_source() {
        let mut a = app();
        carry(&mut a, sample());
        let mut h = harness(a);
        h.state_mut().open_dialog(NAME, json!({}));
        h.run_steps(6);
        h.get_by_label(INTRO);
        h.get_by_label_contains("\"Sales\" in ");
        h.get_by_label_contains("2 modules. It refers to: stdole.");
        // The first module is shown: what it is, what is in it, and its lines.
        h.get_by_label("Totals  ·  Module  ·  8 lines");
        h.get_by_label("2 procedures: AddUp, GetTickCount (in kernel32, outside the workbook).");
        h.get_by_label_contains("Public Sub AddUp()");
        h.get_by_label_contains("MsgBox \"Total: \" & WorksheetFunction.Sum(Range(\"A1:A10\"))");
        // The person chooses another.
        h.get_by_label("ThisWorkbook").click();
        h.run_steps(4);
        h.get_by_label("ThisWorkbook  ·  Workbook or sheet  ·  3 lines");
        h.get_by_label("1 procedure: Workbook_Open (Excel runs it by itself).");
        h.get_by_label_contains("Call AddUp");
        assert!(h.query_by_label_contains("Public Sub AddUp()").is_none());
        // A workbook without macros, and one whose project can't be read, say so.
        h.state_mut().dialog = None;
        carry(h.state_mut(), b"damaged".to_vec());
        h.state_mut().open_dialog(NAME, json!({}));
        h.run_steps(6);
        h.get_by_label_contains("could not be read");
        h.state_mut().dialog = None;
        let doc = h.state_mut().session.active_mut().unwrap();
        Arc::make_mut(&mut doc.wb).vba_project = None;
        h.state_mut().open_dialog(NAME, json!({}));
        h.run_steps(6);
        h.get_by_label(NONE);
    }

    #[test]
    fn what_would_not_show_in_a_macro_is_written_out() {
        let mut a = app();
        let hidden = "Sub Quiet()\r\n    ' a comment\rShell \"calc\"\r\nEnd Sub\r\n";
        carry(&mut a, vba::testing::project("P", &[("Quiet", Kind::Standard, hidden)]));
        let mut h = harness(a);
        h.state_mut().open_dialog(NAME, json!({}));
        h.run_steps(6);
        h.get_by_label_contains("' a comment⟨U+000D⟩Shell \"calc\"");
    }
}
