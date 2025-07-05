use super::page_template::{PAGE_BOTTOM, PAGE_CONTENT, PAGE_TOP};
use pages_components::ButtonComponent;
use pages_components::TaskCard001;

pub fn print_page(
    page_level_data: &[String],
    css_files_markup: &[String],
    components_markup: &[String],
    js_files_markup: &[String],
) -> String {
    let button = ButtonComponent::new(
        vec![String::from("button_component.css")],
        vec![String::from("button_component.js")],
        Some(vec![String::from("button_component_head.js")]),
        None,
        None,
    );

    let task_card: TaskCard001 = TaskCard001::new(
      vec![String::from("task_card_001.css")],
      vec![String::from("task_card_001.js")],
      None,
      None,
      None,
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
        "{}\n{}\n{}\n",
        PAGE_TOP,
        css_files_markup = all_css_markup,
        js_files_markup = all_head_js_markup // Assuming PAGE_TOP has a placeholder for head JS
    );

    let page_bottom_with_js = format!(
        "{}\n{}\n",
        PAGE_BOTTOM,
        js_files_markup = all_bottom_js_markup
    );

    let task_card_markup = task_card.print_component_markup();

    // Step 2: Format PAGE_CONTENT with its specific variables.
    let page_content_with_data_components = format!(
        "
        {}\n
        {}\n
        {}\n
        {}\n
        ",
        PAGE_CONTENT,
        page_level_data = page_data_joined,
        components_markup = all_components_markup,
        task_card_markup = task_card_markup
    );

    // Step 3: Combine all the already formatted parts.
    format!(
        "{}\n{}\n{}",
        page_top_with_css_and_head_js,
        page_content_with_data_components,
        page_bottom_with_js
    )
}