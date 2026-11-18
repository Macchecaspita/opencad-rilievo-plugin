//! Interface language: the 23 languages of Open CAD Studio, following its
//! setting; English otherwise.
//!
//! The host does not pass its UI language to plugins, so it is read from the
//! host's own settings file, `<config>/OpenCADStudio/settings.json`, key
//! `settings.language` (`"it-IT"`, `"en-US"`, … or `"system"`). With
//! `"system"` (or no file) the operating system's locale decides. Any
//! other language gets English.
//!
//! `RV_LANG <code>|AUTO` (`it`, `en`, `de`, `zh-CN`, …) overrides the choice; the override is kept in
//! `<config>/OpenCADStudio/rilievovani-lang.txt`. The ribbon is built once
//! when the plugin loads, so its labels follow a change after a restart;
//! messages and prompts follow it at once.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};

/// Pick the form for the current language: `tr!(it, en, es, fr, tr, pt, pl)`.
/// Every form is an expression, so `format!` works in each and only the
/// chosen one is evaluated. The other languages of Open CAD Studio come from
/// `i18n/<code>.json` at build time (see the `tr_macro` crate); a sentence
/// not translated there falls back to English.
#[macro_export]
macro_rules! tr {
    ($($t:tt)*) => {
        $crate::i18n::tr_impl!($($t)*)
    };
}

#[doc(hidden)]
pub use tr_macro::tr_impl;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    It,
    En,
    Es,
    Fr,
    Tr,
    Pt,
    Pl,
    Bg,
    Cs,
    De,
    El,
    Fi,
    Hi,
    Hu,
    Ja,
    Ko,
    Lo,
    Nl,
    Ru,
    Vi,
    ZhCn,
    ZhTw,
    Ar,
}

pub const ALL: [Lang; 23] = [
    Lang::It,
    Lang::En,
    Lang::Es,
    Lang::Fr,
    Lang::Tr,
    Lang::Pt,
    Lang::Pl,
    Lang::Bg,
    Lang::Cs,
    Lang::De,
    Lang::El,
    Lang::Fi,
    Lang::Hi,
    Lang::Hu,
    Lang::Ja,
    Lang::Ko,
    Lang::Lo,
    Lang::Nl,
    Lang::Ru,
    Lang::Vi,
    Lang::ZhCn,
    Lang::ZhTw,
    Lang::Ar,
];

impl Lang {
    /// Code, as typed after `RV_LANG` and kept in the override file.
    pub fn code(self) -> &'static str {
        match self {
            Lang::It => "it",
            Lang::En => "en",
            Lang::Es => "es",
            Lang::Fr => "fr",
            Lang::Tr => "tr",
            Lang::Pt => "pt",
            Lang::Pl => "pl",
            Lang::Bg => "bg",
            Lang::Cs => "cs",
            Lang::De => "de",
            Lang::El => "el",
            Lang::Fi => "fi",
            Lang::Hi => "hi",
            Lang::Hu => "hu",
            Lang::Ja => "ja",
            Lang::Ko => "ko",
            Lang::Lo => "lo",
            Lang::Nl => "nl",
            Lang::Ru => "ru",
            Lang::Vi => "vi",
            Lang::ZhCn => "zh-CN",
            Lang::ZhTw => "zh-TW",
            Lang::Ar => "ar",
        }
    }

    /// The language's own name.
    pub fn name(self) -> &'static str {
        match self {
            Lang::It => "italiano",
            Lang::En => "English",
            Lang::Es => "español",
            Lang::Fr => "français",
            Lang::Tr => "Türkçe",
            Lang::Pt => "português",
            Lang::Pl => "polski",
            Lang::Bg => "български",
            Lang::Cs => "čeština",
            Lang::De => "Deutsch",
            Lang::El => "Ελληνικά",
            Lang::Fi => "suomi",
            Lang::Hi => "हिन्दी",
            Lang::Hu => "magyar",
            Lang::Ja => "日本語",
            Lang::Ko => "한국어",
            Lang::Lo => "ລາວ",
            Lang::Nl => "Nederlands",
            Lang::Ru => "русский",
            Lang::Vi => "Tiếng Việt",
            Lang::ZhCn => "简体中文",
            Lang::ZhTw => "繁體中文",
            Lang::Ar => "العربية",
        }
    }

    fn id(self) -> u8 {
        ALL.iter().position(|&l| l == self).unwrap() as u8 + 1
    }

    /// From a tag such as `"it"`, `"pt-BR"`, `"fr_CH.UTF-8"`, `"zh-TW"`.
    pub fn from_tag(tag: &str) -> Option<Lang> {
        let t = tag.trim().to_ascii_lowercase().replace('_', "-");
        let lang = t.split(['-', '.', '@']).next().unwrap_or("");
        if lang == "zh" {
            let traditional = ["-tw", "-hk", "-mo", "-hant"].iter().any(|s| t.contains(s));
            return Some(if traditional { Lang::ZhTw } else { Lang::ZhCn });
        }
        ALL.into_iter().find(|l| l.code() == lang)
    }
}

/// The user's explicit choice from `RV_LANG`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    Auto,
    Fixed(Lang),
}

const UNSET: u8 = 0;

/// Resolved language, cached until [`refresh`].
static CURRENT: AtomicU8 = AtomicU8::new(UNSET);

/// Whether messages should be in Italian.
pub fn italian() -> bool {
    current() == Lang::It
}

/// Whether numbers are written with a decimal comma (as is usual in the
/// language) rather than a point.
pub fn decimal_comma() -> bool {
    !matches!(current(), Lang::En | Lang::Hi | Lang::Ja | Lang::Ko | Lang::ZhCn | Lang::ZhTw | Lang::Ar)
}

pub fn current() -> Lang {
    let v = CURRENT.load(Ordering::Relaxed);
    match ALL.get((v as usize).wrapping_sub(1)) {
        Some(&l) => l,
        None => {
            let l = resolve();
            store(l);
            l
        }
    }
}

/// Re-read the settings (called before each command, so a language changed
/// in the CAD is picked up without restarting).
pub fn refresh() {
    store(resolve());
}

/// Use this language until the next [`refresh`] (tests).
#[doc(hidden)]
pub fn force(l: Lang) {
    store(l);
}

fn store(l: Lang) {
    CURRENT.store(l.id(), Ordering::Relaxed);
}

/// The current choice and the language it leads to.
pub fn status() -> (Choice, Lang) {
    (read_choice(), current())
}

/// Save a `RV_LANG` choice and apply it.
pub fn set_choice(c: Choice) -> std::io::Result<()> {
    let path = choice_path().ok_or_else(|| std::io::Error::other("config folder not found"))?;
    match c {
        Choice::Auto => match std::fs::remove_file(&path) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(e),
            _ => {}
        },
        Choice::Fixed(l) => {
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)?;
            }
            std::fs::write(&path, format!("{}\n", l.code()))?;
        }
    }
    refresh();
    Ok(())
}

fn resolve() -> Lang {
    match read_choice() {
        Choice::Fixed(l) => l,
        Choice::Auto => {
            let tag = host_language().filter(|t| !t.eq_ignore_ascii_case("system")).or_else(os_locale);
            tag.and_then(|t| Lang::from_tag(&t)).unwrap_or(Lang::En)
        }
    }
}

/// `"it"`, `"it-IT"`, `"it_CH.UTF-8"`, … — but not `"ita"`-less strings.
pub fn is_italian_tag(tag: &str) -> bool {
    Lang::from_tag(tag) == Some(Lang::It)
}

/// Open CAD Studio's per-user config folder (same rules as the host).
pub(crate) fn config_dir() -> Option<PathBuf> {
    // Unset and empty variables are treated alike.
    let var = |k: &str| std::env::var_os(k).filter(|v| !v.is_empty());
    let base: PathBuf = if cfg!(target_os = "windows") {
        var("APPDATA").map(PathBuf::from)?
    } else if cfg!(target_os = "macos") {
        let mut p = PathBuf::from(var("HOME")?);
        p.push("Library");
        p.push("Application Support");
        p
    } else if let Some(d) = var("XDG_CONFIG_HOME") {
        PathBuf::from(d)
    } else {
        let mut p = PathBuf::from(var("HOME")?);
        p.push(".config");
        p
    };
    Some(base.join("OpenCADStudio"))
}

fn choice_path() -> Option<PathBuf> {
    Some(config_dir()?.join("rilievovani-lang.txt"))
}

fn read_choice() -> Choice {
    let Some(p) = choice_path() else { return Choice::Auto };
    match std::fs::read_to_string(p) {
        Ok(s) => parse_choice(&s).unwrap_or(Choice::Auto),
        Err(_) => Choice::Auto,
    }
}

pub fn parse_choice(s: &str) -> Option<Choice> {
    let l = match s.trim().to_lowercase().as_str() {
        "auto" | "" => return Some(Choice::Auto),
        "it" | "ita" | "italiano" | "italian" => Lang::It,
        "en" | "eng" | "english" | "inglese" => Lang::En,
        "es" | "spa" | "español" | "espanol" | "spanish" | "spagnolo" => Lang::Es,
        "fr" | "fra" | "français" | "francais" | "french" | "francese" => Lang::Fr,
        "tr" | "tur" | "türkçe" | "turkce" | "turkish" | "turco" => Lang::Tr,
        "pt" | "por" | "português" | "portugues" | "portuguese" | "portoghese" => Lang::Pt,
        "pl" | "pol" | "polski" | "polish" | "polacco" => Lang::Pl,
        "de" | "deu" | "deutsch" | "german" | "tedesco" => Lang::De,
        "nl" | "nld" | "nederlands" | "dutch" | "olandese" => Lang::Nl,
        "ru" | "rus" | "русский" | "russian" | "russo" => Lang::Ru,
        "zh" | "zh-cn" | "zh_cn" | "chinese" | "cinese" => Lang::ZhCn,
        "zh-tw" | "zh_tw" => Lang::ZhTw,
        other => return Lang::from_tag(other).map(Choice::Fixed),
    };
    Some(Choice::Fixed(l))
}

/// `settings.language` from the host's settings.json.
fn host_language() -> Option<String> {
    let text = std::fs::read_to_string(config_dir()?.join("settings.json")).ok()?;
    language_from_settings(&text)
}

pub fn language_from_settings(json: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    v.get("settings")?.get("language")?.as_str().map(str::to_string)
}

/// The operating system's UI locale, e.g. `"it-IT"`.
fn os_locale() -> Option<String> {
    #[cfg(windows)]
    {
        #[link(name = "kernel32")]
        extern "system" {
            fn GetUserDefaultLocaleName(name: *mut u16, len: i32) -> i32;
        }
        let mut buf = [0u16; 85];
        // SAFETY: the buffer is valid for 85 UTF-16 units, the documented maximum.
        let n = unsafe { GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32) };
        if n > 1 {
            return Some(String::from_utf16_lossy(&buf[..(n as usize - 1)]));
        }
        None
    }
    #[cfg(not(windows))]
    {
        ["LC_ALL", "LC_MESSAGES", "LANG"]
            .iter()
            .filter_map(|k| std::env::var(k).ok())
            .find(|v| !v.is_empty() && v != "C" && v != "POSIX")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn italian_tags() {
        for t in ["it", "it-IT", "it_CH.UTF-8", "IT-it"] {
            assert!(is_italian_tag(t), "{t}");
        }
        for t in ["en-US", "de-DE", "system", "", "iterate"] {
            assert!(!is_italian_tag(t), "{t}");
        }
        assert_eq!(Lang::from_tag("pt-BR"), Some(Lang::Pt));
        assert_eq!(Lang::from_tag("tr_TR.UTF-8"), Some(Lang::Tr));
        assert_eq!(Lang::from_tag("pl"), Some(Lang::Pl));
        assert_eq!(Lang::from_tag("fr-CA"), Some(Lang::Fr));
        assert_eq!(Lang::from_tag("es-419"), Some(Lang::Es));
        assert_eq!(Lang::from_tag("de-DE"), Some(Lang::De));
        assert_eq!(Lang::from_tag("zh-TW"), Some(Lang::ZhTw));
        assert_eq!(Lang::from_tag("zh_CN.UTF-8"), Some(Lang::ZhCn));
        assert_eq!(Lang::from_tag("xx-YY"), None);
        for l in ALL {
            assert_eq!(ALL[l.id() as usize - 1], l);
        }
    }

    #[test]
    fn reads_host_settings() {
        let j = r#"{"settings":{"osmode":4,"language":"it-IT","spell":{"language":"en-US"}},"theme":{}}"#;
        assert_eq!(language_from_settings(j).as_deref(), Some("it-IT"));
        assert_eq!(language_from_settings(r#"{"settings":{"language":"system"}}"#).as_deref(), Some("system"));
        assert_eq!(language_from_settings(r#"{"settings":{}}"#), None);
        assert_eq!(language_from_settings("not json"), None);
    }

    #[test]
    fn choices() {
        assert_eq!(parse_choice("it"), Some(Choice::Fixed(Lang::It)));
        assert_eq!(parse_choice(" English\n"), Some(Choice::Fixed(Lang::En)));
        assert_eq!(parse_choice("PL"), Some(Choice::Fixed(Lang::Pl)));
        assert_eq!(parse_choice("Türkçe"), Some(Choice::Fixed(Lang::Tr)));
        assert_eq!(parse_choice("auto"), Some(Choice::Auto));
        assert_eq!(parse_choice("klingon"), None);
    }
}
