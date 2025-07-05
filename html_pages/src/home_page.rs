// html_pages/src/home_page.rs

use super::page_template::{PAGE_BOTTOM, PAGE_CONTENT, PAGE_TOP};
use pages_components::ButtonComponent; // Import the component struct

pub fn print_page(
    page_level_data: &[String],
    css_files_markup: &[String],
    components_markup: &[String], // This will now include actual component HTML
    js_files_markup: &[String],
) -> String {
    // Initialize a ButtonComponent (dummy arguments for now)
    let button = ButtonComponent::new(
        vec![String::from("button_component.css")], // CSS file name from the component
        vec![String::from("button_component.js")],  // JS file name from the component (body bottom)
        Some(vec![String::from("button_component_head.js")]), // JS file name for head
        None, // No session object for now
        None, // No data object for now
    );

    // Get the component's markup and its associated head/bottom JS/CSS
    let button_markup = button.print_component_markup();
    let button_styles = button.print_styles_head_tags();
    let button_head_js = button.print_javascript_head_tags();
    let button_bottom_js = button.print_javascript_bottom_tags();

    // Combine all CSS markup (global + component)
    let all_css_markup = format!("{}\n{}", css_files_markup.join("\n    "), button_styles);

    // Combine all JS head markup (if any)
    let all_head_js_markup = button_head_js; // For now, only button_component_head.js

    // Combine all JS bottom markup (global + component)
    let all_bottom_js_markup = format!("{}\n{}", js_files_markup.join("\n    "), button_bottom_js);

    // Combine all component markup
    let all_components_markup = format!("{}\n{}", components_markup.join("\n"), button_markup);


    let page_data_joined = page_level_data.join("\n");

    // Step 1: Format PAGE_TOP and PAGE_BOTTOM by filling their internal placeholders.
    let page_top_with_css_and_head_js = format!(
        PAGE_TOP,
        css_files_markup = all_css_markup,
        js_files_markup = all_head_js_markup // Assuming PAGE_TOP has a placeholder for head JS
    );

    let page_bottom_with_js = format!(
        PAGE_BOTTOM,
        js_files_markup = all_bottom_js_markup
    );

    // Step 2: Format PAGE_CONTENT with its specific variables.
    let page_content_with_data_components = format!(
        PAGE_CONTENT,
        page_level_data = page_data_joined,
        components_markup = all_components_markup
    );

    // Step 3: Combine all the already formatted parts.
    format!(
        "{}\n{}\n{}",
        page_top_with_css_and_head_js,
        page_content_with_data_components,
        page_bottom_with_js
    )
}