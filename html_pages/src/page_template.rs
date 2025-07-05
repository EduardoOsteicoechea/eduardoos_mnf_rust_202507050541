// html_pages/src/page_template.rs

// These are constants holding parts of your HTML template.
// 'static lifetime means they are valid for the entire duration of the program.
// They are string slices (&str) which are efficient for static content.

// PAGE_TOP: Contains the DOCTYPE, html, head, and opening body tag.
// It includes a placeholder for CSS files.
pub const PAGE_TOP: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust SSR Application</title>
</head>
<body>
"#;

// PAGE_CONTENT: This is where the main content of your specific page will go.
// It includes placeholders for page-level data and components.
pub const PAGE_CONTENT: &str = r#"
    <header>
        <h1>Welcome to the Modular Rust SSR App!</h1>
    </header>
    <main>
        <h2>Page Content Here</h2>
    </main>
"#;

// PAGE_BOTTOM: Contains the closing body and html tags, and a placeholder for JS files.
pub const PAGE_BOTTOM: &str = r#"
</body>
</html>
"#;