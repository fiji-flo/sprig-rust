#[macro_use]
pub mod utils;
pub mod defaults;
pub mod strings;
pub mod time;

use gtmpl::Func;

/// SPRIG functions.
pub static SPRIG: &[(&str, Func)] = &[
    // strings
    ("base64encode", strings::base64encode as Func),
    ("base64decode", strings::base64decode as Func),
    ("base32encode", strings::base32encode as Func),
    ("base32decode", strings::base32decode as Func),
    ("abbrev", strings::abbrev as Func),
    ("abbrevboth", strings::abbrevboth as Func),
    ("upper", strings::upper as Func),
    ("lower", strings::lower as Func),
    ("initials", strings::initials as Func),
    ("randAlphaNumeric", strings::rand_alpha_numeric as Func),
    ("randAlpha", strings::rand_alpha as Func),
    ("randAscii", strings::rand_ascii as Func),
    ("randNumeric", strings::rand_numeric as Func),
    ("untitle", strings::untitle as Func),
    ("replace", strings::replace as Func),
    ("plural", strings::plural as Func),
    ("trunc", strings::trunc as Func),
    ("join", strings::join as Func),
    ("split", strings::split as Func),
    ("substring", strings::substring as Func),
    ("trim", strings::trim as Func),
    ("trimAll", strings::trim_all as Func),
    ("trimSuffix", strings::trim_suffix as Func),
    ("trimPrefix", strings::trim_prefix as Func),
    ("contains", strings::contains as Func),
    ("hasSuffix", strings::has_suffix as Func),
    ("hasPrefix", strings::has_prefix as Func),
    ("nospace", strings::nospace as Func),
    ("repeat", strings::repeat as Func),
    ("indent", strings::indent as Func),
    ("nindent", strings::nindent as Func),
    // time
    ("now", time::now as Func),
    // defaults
    ("default", defaults::default as Func),
];
