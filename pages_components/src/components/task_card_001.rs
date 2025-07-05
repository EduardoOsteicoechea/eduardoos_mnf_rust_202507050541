// pages_components/src/components/task_card_001.rs

pub struct TaskCard001 {
    component_markup: &'static str,
    css_file_names: Vec<String>,
    js_file_names: Vec<String>,
    head_js_file_names: Option<Vec<String>>,
    session_object: Option<String>,
    data_object: Option<String>,
    component_id: String,
    sub_component_ids: Vec<String>,
}

impl TaskCard001 {
    pub fn new(
        css_file_names: Vec<String>,
        js_file_names: Vec<String>,
        head_js_file_names: Option<Vec<String>>,
        session_object: Option<String>,
        data_object: Option<String>,
    ) -> Self {
        let component_markup = include_str!("task_card_001/task_card_001.html");

        let component_id: String = "task_card_001".to_string();
        let sub_component_ids: Vec<String> = [
          "task_card_001_header",
          "task_card_001_body",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
        ;

        Self {
            component_markup,
            css_file_names,
            js_file_names,
            head_js_file_names,
            session_object,
            data_object,
            component_id,
            sub_component_ids
        }
    }

    pub fn print_component_markup(&self) -> String {
        self.component_markup.to_string()
    }

    pub fn print_styles_head_tags(&self) -> String {
        self.css_file_names
            .iter()
            .map(|file_name| format!(r#"<link rel="stylesheet" href="/{}">"#, file_name))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn print_javascript_head_tags(&self) -> String {
        self.head_js_file_names
            .as_ref() // Get an Option<&Vec<String>>
            .map_or(String::new(), |files| { // If Some, map; otherwise, empty string
                files.iter()
                    .map(|file_name| format!(r#"<script src="/{}"></script>"#, file_name))
                    .collect::<Vec<String>>()
                    .join("\n")
            })
    }

    pub fn print_javascript_bottom_tags(&self) -> String {
        self.js_file_names
            .iter()
            .map(|file_name| format!(r#"<script src="/{}"></script>"#, file_name))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn print_component_ids(&self) -> String {
        self.component_id.clone()
    }
}