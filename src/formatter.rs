use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::util::as_24_bit_terminal_escaped;

pub fn print_highlighted_json(text: &str) {
    print!("{}", format_highlighted_string(text, "json"));
}

pub fn print_highlighted_xml(text: &str) {
    print!("{}", format_highlighted_string(text, "xml"));
}

pub fn format_highlighted_string(text: &str, extension: &str) -> String {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(extension)
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    let theme = &ts.themes["base16-ocean.dark"];
    let mut h = HighlightLines::new(syntax, theme);
    let mut output = String::new();

    for line in text.lines() {
        if let Ok(ranges) = h.highlight_line(&(line.to_string() + "\n"), &ps) {
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            output.push_str(&escaped);
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_highlighted_string_json() {
        let json_text = "{\"name\": \"Anilcan\"}";
        let highlighted = format_highlighted_string(json_text, "json");
        assert!(!highlighted.is_empty());
    }
}
