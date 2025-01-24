use std::collections::HashMap;

pub const DARK_MODE: &str = "--dark-mode";
pub const LIGHT_MODE: &str = "--light-mode";

/// Extracts property sets prefixed with `--` from the given CSS, as well as the remaining CSS.
pub fn extract_prefixed_property_sets(css: &str) -> (HashMap<String, String>, String) {
    let mut property_sets: HashMap<String, String> = HashMap::new();
    let mut remaining_css = String::new();

    let mut current_set: Option<String> = None;
    let mut brace_count = 0;
    let mut current_block = String::new();

    for line in css.lines() {
        let trimmed = line.trim();

        // Count braces
        brace_count += trimmed.chars().filter(|&c| c == '{').count();
        brace_count -= trimmed.chars().filter(|&c| c == '}').count();

        // Check for new property set
        if trimmed.starts_with("--") && trimmed.contains('{') {
            let set_name = trimmed.split('{').next().unwrap().trim().to_string();
            current_set = Some(set_name);
            continue;
        }

        if let Some(ref set_name) = current_set {
            if brace_count == 0 {
                // End of block
                property_sets.insert(set_name.clone(), current_block.trim_end().to_string());
                current_block.clear();
                current_set = None;
            } else {
                current_block.push_str(line);
                current_block.push('\n');
            }
        } else {
            remaining_css.push_str(line);
            remaining_css.push('\n');
        }
    }

    (property_sets, remaining_css.trim().to_string())
}

/// Parses CSS rules into a HashMap of property names to values.
/// Each rule should be in the format "property-name: value;"
///
/// Note that it will not handle comments correctly.
pub fn parse_rules(rules: &str) -> HashMap<String, String> {
    rules
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_once(':'))
        .map(|(property, value)| {
            (
                property.trim().to_string(),
                value.trim().trim_end_matches(';').trim().to_string(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_css_modes() {
        let input = r#"--dark-mode {
  --color: white;
  --color-secondary: #CCC;
  --background-color: #3C2954;
}

--light-mode {
  --color: black;
  --color-secondary: #333;
  --background-color: #C5B4DB;
}

:root {
  --header-height: 64px;
}

body {
  max-width: 1100px;
  margin: auto;
  color: var(--color);
  background-color: var(--background-color);
}"#;

        let expected_dark_mode = r#"  --color: white;
  --color-secondary: #CCC;
  --background-color: #3C2954;"#;

        let expected_light_mode = r#"  --color: black;
  --color-secondary: #333;
  --background-color: #C5B4DB;"#;

        let expected_remaining = r#":root {
  --header-height: 64px;
}

body {
  max-width: 1100px;
  margin: auto;
  color: var(--color);
  background-color: var(--background-color);
}"#;

        let (property_sets, remaining) = extract_prefixed_property_sets(input);

        assert_eq!(
            property_sets.get("--dark-mode").map(|s| s.as_str()),
            Some(expected_dark_mode)
        );
        assert_eq!(
            property_sets.get("--light-mode").map(|s| s.as_str()),
            Some(expected_light_mode)
        );
        assert_eq!(remaining, expected_remaining);
    }

    #[test]
    fn test_parse_rules() {
        let input = r#"  --color: black;
  --color-secondary: #333;
  --background-color: #C5B4DB;
"#;

        let rules = parse_rules(input);

        assert_eq!(rules.get("--color").unwrap(), "black");
        assert_eq!(rules.get("--color-secondary").unwrap(), "#333");
        assert_eq!(rules.get("--background-color").unwrap(), "#C5B4DB");
    }
}
