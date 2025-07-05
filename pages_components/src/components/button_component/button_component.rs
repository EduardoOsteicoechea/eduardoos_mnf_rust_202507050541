// pages_components/src/components/button_component/button_component.rs

// Rust doesn't have "classes" in the C# sense, but structs with methods
// provide similar object-like behavior.

/// Represents a Button UI Component.
/// This struct holds the component's HTML markup and its associated
/// static file names (CSS, JS).
pub struct ButtonComponent {
    // The raw HTML content for this component.
    // `include_str!` reads the file at compile time, embedding its content
    // directly into the binary. This is highly performant for static HTML.
    component_markup: &'static str,
    // Paths to CSS files specific to this component.
    css_file_names: Vec<String>,
    // Paths to JavaScript files specific to this component, for body bottom.
    js_file_names: Vec<String>,
    // Paths to JavaScript files specific to this component, for head.
    head_js_file_names: Option<Vec<String>>,
    // Optional session data (placeholder for now)
    session_object: Option<String>,
    // Optional component-specific data (placeholder for now)
    data_object: Option<String>,
    // Unique ID for this component instance, useful for JS interaction
    component_id: String,
}

impl ButtonComponent {
    /// Constructor for the ButtonComponent.
    ///
    /// Reads the component's HTML markup from `button_component.html` at compile time.
    /// Initializes the component with provided CSS and JS file names, and optional data.
    ///
    /// Arguments:
    /// - `css_file_names`: A list of CSS file names (e.g., "button_component.css").
    /// - `js_file_names`: A list of JS file names for the body bottom (e.g., "button_component.js").
    /// - `head_js_file_names`: Optional list of JS file names for the head.
    /// - `session_object`: Optional session data as a string.
    /// - `data_object`: Optional component-specific data as a string.
    ///
    /// Returns:
    /// A new `ButtonComponent` instance.
    pub fn new(
        css_file_names: Vec<String>,
        js_file_names: Vec<String>,
        head_js_file_names: Option<Vec<String>>,
        session_object: Option<String>,
        data_object: Option<String>,
    ) -> Self {
        // Read the HTML file content at compile time.
        // The path is relative to the `button_component.rs` file.
        let component_markup = include_str!("button_component.html");

        // Generate a simple unique ID for this component instance.
        // In a real app, you might use a UUID generator or a more robust system.
        let component_id = format!("button-{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("default"));


        Self {
            component_markup,
            css_file_names,
            js_file_names,
            head_js_file_names,
            session_object,
            data_object,
            component_id,
        }
    }

    /// Returns the raw HTML markup for the component.
    pub fn print_component_markup(&self) -> String {
        // You might want to inject `component_id` or other data into the markup here
        // using `format!`, but for simplicity, we return the raw string for now.
        // Example: self.component_markup.replace("{component_id}", &self.component_id)
        self.component_markup.to_string()
    }

    /// Generates HTML `<link>` tags for the component's CSS files.
    /// These are typically placed in the `<head>` section of the HTML.
    pub fn print_styles_head_tags(&self) -> String {
        self.css_file_names
            .iter()
            .map(|file_name| format!(r#"<link rel="stylesheet" href="/static/{}">"#, file_name))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Generates HTML `<script>` tags for JavaScript files to be placed in the `<head>`.
    pub fn print_javascript_head_tags(&self) -> String {
        self.head_js_file_names
            .as_ref() // Get an Option<&Vec<String>>
            .map_or(String::new(), |files| { // If Some, map; otherwise, empty string
                files.iter()
                    .map(|file_name| format!(r#"<script src="/static/{}"></script>"#, file_name))
                    .collect::<Vec<String>>()
                    .join("\n")
            })
    }

    /// Generates HTML `<script>` tags for JavaScript files to be placed at the bottom of `<body>`.
    pub fn print_javascript_bottom_tags(&self) -> String {
        self.js_file_names
            .iter()
            .map(|file_name| format!(r#"<script src="/static/{}"></script>"#, file_name))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Returns a string representation of the component's unique ID.
    /// This could be used for JavaScript to target specific component instances.
    pub fn print_component_ids(&self) -> String {
        self.component_id.clone()
    }
}