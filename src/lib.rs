use std::sync::Arc;

use resvg::tiny_skia::{self, Pixmap};
use serde::{Deserialize, Serialize};
use usvg::fontdb;

wit_bindgen::generate!({
    world: "slipway",
});

struct Component;

export!(Component);

impl Guest for Component {
    fn run(input: String) -> Result<String, ComponentError> {
        let input: Input = serde_json::from_str(&input).expect("should parse JSON from stdin");

        let mut pixels = Pixmap::new(input.width, input.height).ok_or(ComponentError {
            message: format!(
                "Failed to create pixmap with dimensions {}x{}",
                input.width, input.height
            ),
            inner: vec![],
        })?;

        let options = usvg::Options {
            font_resolver: create_font_resolver(),
            ..usvg::Options::default()
        };

        let tree: usvg::Tree =
            usvg::Tree::from_data(input.svg.as_bytes(), &options).map_err(|error| {
                ComponentError {
                    message: error.to_string(),
                    inner: vec![],
                }
            })?;

        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixels.as_mut());

        let output = Output {
            canvas: CanvasResult {
                width: input.width,
                height: input.height,
                data: slipway_host::encode_bin(pixels.take().as_slice()),
            },
        };

        Ok(serde_json::to_string(&output).expect("should serialize output to JSON"))
    }
}

pub fn create_font_resolver<'a>() -> usvg::FontResolver<'a> {
    usvg::FontResolver {
        select_font: slipway_font_selector(),
        select_fallback: usvg::FontResolver::default_fallback_selector(),
    }
}

const SERIF_STR: &str = "serif";
const SANS_SERIF_STR: &str = "sans-serif";
const CURSIVE_STR: &str = "cursive";
const FANTASY_STR: &str = "fantasy";
const MONOSPACE_STR: &str = "monospace";

// Create a static mutable hash set of strings to store requested fonts.
static REQUESTED_FONTS: std::sync::LazyLock<std::sync::Mutex<std::collections::HashSet<String>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashSet::new()));

pub fn slipway_font_selector() -> usvg::FontSelectionFn<'static> {
    Box::new(move |font, fontdb| {
        let mut name_list = Vec::new();
        for family in font.families() {
            let (family_str, family) = match family {
                usvg::FontFamily::Serif => (SERIF_STR, fontdb::Family::Serif),
                usvg::FontFamily::SansSerif => (SANS_SERIF_STR, fontdb::Family::SansSerif),
                usvg::FontFamily::Cursive => (CURSIVE_STR, fontdb::Family::Cursive),
                usvg::FontFamily::Fantasy => (FANTASY_STR, fontdb::Family::Fantasy),
                usvg::FontFamily::Monospace => (MONOSPACE_STR, fontdb::Family::Monospace),
                usvg::FontFamily::Named(s) => match s.as_ref() {
                    SERIF_STR => (SERIF_STR, fontdb::Family::Serif),
                    SANS_SERIF_STR => (SANS_SERIF_STR, fontdb::Family::SansSerif),
                    CURSIVE_STR => (CURSIVE_STR, fontdb::Family::Cursive),
                    FANTASY_STR => (FANTASY_STR, fontdb::Family::Fantasy),
                    MONOSPACE_STR => (MONOSPACE_STR, fontdb::Family::Monospace),
                    _ => (s.as_str(), fontdb::Family::Name(s)),
                },
            };

            name_list.push(family);

            {
                let mut requested_fonts = REQUESTED_FONTS
                    .lock()
                    .expect("should be able to acquire lock");

                if requested_fonts.contains(family_str) {
                    continue;
                }

                requested_fonts.insert(family_str.to_string());
            }

            let query = fontdb::Query {
                families: &[family],
                weight: fontdb::Weight::default(),
                stretch: fontdb::Stretch::default(),
                style: fontdb::Style::default(),
            };

            let id = fontdb.query(&query);

            if id.is_none() {
                slipway_host::log_debug(&format!(
                    "No match for \"{}\" font-family. Requesting from host.",
                    family_str,
                ));
                let maybe_resolved_font = slipway_host::font(family_str);
                if let Some(resolved_font) = maybe_resolved_font {
                    slipway_host::log_debug(&format!(
                        "Host resolved as \"{}\".",
                        resolved_font.family
                    ));
                    let fontdb_mut = Arc::make_mut(fontdb);
                    fontdb_mut.load_font_data(resolved_font.data);

                    match family {
                        fontdb::Family::Serif => fontdb_mut.set_serif_family(&resolved_font.family),
                        fontdb::Family::SansSerif => {
                            fontdb_mut.set_sans_serif_family(&resolved_font.family)
                        }
                        fontdb::Family::Cursive => {
                            fontdb_mut.set_cursive_family(&resolved_font.family)
                        }
                        fontdb::Family::Fantasy => {
                            fontdb_mut.set_fantasy_family(&resolved_font.family)
                        }
                        fontdb::Family::Monospace => {
                            fontdb_mut.set_monospace_family(&resolved_font.family)
                        }
                        fontdb::Family::Name(_) => {}
                    }
                } else {
                    slipway_host::log_warn(&format!(
                        "No host match for \"{}\" font-family.",
                        family_str,
                    ));
                }
            }
        }

        let stretch = match font.stretch() {
            usvg::FontStretch::UltraCondensed => fontdb::Stretch::UltraCondensed,
            usvg::FontStretch::ExtraCondensed => fontdb::Stretch::ExtraCondensed,
            usvg::FontStretch::Condensed => fontdb::Stretch::Condensed,
            usvg::FontStretch::SemiCondensed => fontdb::Stretch::SemiCondensed,
            usvg::FontStretch::Normal => fontdb::Stretch::Normal,
            usvg::FontStretch::SemiExpanded => fontdb::Stretch::SemiExpanded,
            usvg::FontStretch::Expanded => fontdb::Stretch::Expanded,
            usvg::FontStretch::ExtraExpanded => fontdb::Stretch::ExtraExpanded,
            usvg::FontStretch::UltraExpanded => fontdb::Stretch::UltraExpanded,
        };

        let style = match font.style() {
            usvg::FontStyle::Normal => fontdb::Style::Normal,
            usvg::FontStyle::Italic => fontdb::Style::Italic,
            usvg::FontStyle::Oblique => fontdb::Style::Oblique,
        };

        let query = fontdb::Query {
            families: &name_list,
            weight: fontdb::Weight(font.weight()),
            stretch,
            style,
        };

        let id = fontdb.query(&query);

        if id.is_none() {
            slipway_host::log_warn(&format!(
                "No match for \"{}\" font-family.",
                font.families()
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }

        id
    })
}

#[derive(Deserialize)]
struct Input {
    width: u32,
    height: u32,
    svg: String,
}

#[derive(Serialize)]
struct Output {
    canvas: CanvasResult,
}

#[derive(Serialize, Clone, Debug)]
struct CanvasResult {
    width: u32,
    height: u32,
    data: String,
}
