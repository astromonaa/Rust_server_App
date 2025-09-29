use std::fs;
use std::collections::HashMap;

pub fn render_template(path: &str, vars: &HashMap<&str, &str>) -> Result<String, std::io::Error> {
    let mut content = fs::read_to_string(path)?;
    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key); // превращает "activation_link" -> "{{activation_link}}"
        content = content.replace(&placeholder, value);
    }
    Ok(content)
}
