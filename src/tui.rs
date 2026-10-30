use std::io::Result;

use cursive::{
    Cursive,
    theme::{BorderStyle, Color, PaletteColor},
    view::{Resizable, Scrollable},
    views::{Canvas, Dialog, LinearLayout, SelectView, TextView},
};

use crate::helper::{Targets, gen_template, get_targets};

pub fn start_tui() -> Result<()> {
    let mut siv = cursive::default();

    let mut theme = siv.current_theme().clone();
    theme.shadow = false;
    theme.borders = BorderStyle::Simple;
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::View] = Color::TerminalDefault;
    theme.palette[PaletteColor::Primary] = Color::TerminalDefault;
    theme.palette[PaletteColor::TitlePrimary] = Color::Light(cursive::theme::BaseColor::Cyan);
    theme.palette[PaletteColor::Highlight] = Color::Dark(cursive::theme::BaseColor::Cyan);
    theme.palette[PaletteColor::HighlightText] = Color::Dark(cursive::theme::BaseColor::Black);

    siv.set_theme(theme);
    siv.add_global_callback('q', |s| s.quit());

    page_table(&mut siv)?;

    siv.run();
    Ok(())
}

fn row(a: &str, b: &str, c: &str, d: &str) -> String {
    format!("{:<16} {:<16} {:<16} {}", a, b, c, d)
}

fn page_table(s: &mut Cursive) -> Result<()> {
    let targets: Vec<Targets> = get_targets()?;

    let list = SelectView::new()
        .with_all(targets.into_iter().map(|t| {
            let label = row(
                &t.goos,
                &t.goarch,
                &t.cgosupported.to_string(),
                &t.firstclass.to_string(),
            );
            (label, t)
        }))
        .on_submit(|s, t: &Targets| page_details(s, t));

    let table = LinearLayout::vertical()
        .child(TextView::new(row("OS", "ARCH", "CGO", "FIRSTCLASS")))
        .child(Canvas::new(()).with_draw(|_, p| p.print_hline((0, 0), p.size.x, "─")))
        .child(list.scrollable());

    s.add_layer(Dialog::around(table).title("Targets").full_screen());

    Ok(())
}

fn page_details(s: &mut Cursive, t: &Targets) {
    let goos = t.goos.clone();
    let goarch = t.goarch.clone();
    let cgosupported = t.cgosupported.clone();
    let firstclass = t.firstclass.clone();

    s.add_layer(
        Dialog::new()
            .title(format!("{}/{}", &goos, &goarch))
            .button("Create Template", move |s| {
                match gen_template(
                    goos.clone(),
                    goarch.clone(),
                    cgosupported.clone(),
                    firstclass.clone(),
                ) {
                    Ok(_) => {
                        s.pop_layer();
                        se_dialog(s, "Successfully generated template.".to_string(), 0);
                    }
                    Err(e) => {
                        s.pop_layer();
                        se_dialog(s, e.to_string(), 1);
                    }
                }
            })
            .button("Return", |s| {
                s.pop_layer();
            }),
    );
}

fn se_dialog(s: &mut Cursive, msg: String, errcode: i32) {
    if errcode != 0 {
        s.add_layer(
            Dialog::new()
                .title("Failed!")
                .content(TextView::new(msg).scrollable().max_height(20))
                .button("Exit", |s| s.quit())
                .button("Return", |s| {
                    s.pop_layer();
                }),
        );
    } else {
        s.add_layer(
            Dialog::new()
                .title("Success!")
                .content(TextView::new(msg).scrollable().max_height(20))
                .button("Exit", |s| s.quit())
                .button("Return", |s| {
                    s.pop_layer();
                }),
        );
    }
}
