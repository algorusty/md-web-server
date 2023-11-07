use std::fs;
use std::path::Path;
 
// Function to generate the navbar with github-markdown-css classes for styling
pub async fn generate_navbar(base_path: &Path) -> String {
    let mut navbar_items = vec![];

    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                    // Remove .md and replace underscores with spaces for display
                    let display_name = file_name
                        .trim_end_matches(".md")
                        .replace('_', " ");
                    navbar_items.push(format!(
                        "<li><a href='/{0}' class='nav-link' title='{1}'>{1}</a></li>",
                        file_name, display_name
                    ));
                }
            }
        }
    }

    // Sort the navbar items alphabetically
    navbar_items.sort();

    // Format the list items into an unordered list with GitHub markdown CSS classes
    let navbar_list = format!("<ul class='list-style-none'>{}</ul>", navbar_items.join(""));

    // Complete navbar HTML with GitHub markdown CSS classes
    format!(
        r#"<aside class="sidebar markdown-body">
            <nav class="menu">
            {list}
            </nav>
           </aside>"#,
        list = navbar_list
    )
}
