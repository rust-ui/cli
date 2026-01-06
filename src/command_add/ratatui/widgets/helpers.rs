/// Filter items based on search query (prefix matching)
pub fn filter_items<'a>(items: &[&'a str], search_query: &str) -> Vec<&'a str> {
    if search_query.is_empty() {
        items.to_vec()
    } else {
        items
            .iter()
            .filter(|item| item.to_lowercase().starts_with(&search_query.to_lowercase()))
            .copied()
            .collect()
    }
}

/// Get the currently selected item based on scroll position and search query
pub fn get_selected_item<'a>(items: &[&'a str], scroll: usize, search_query: &str) -> Option<&'a str> {
    let filtered_items = filter_items(items, search_query);

    if !filtered_items.is_empty() && scroll < filtered_items.len() {
        Some(filtered_items[scroll])
    } else {
        None
    }
}

/// Get item at a specific visual index in the filtered list
pub fn get_item_at_visual_index<'a>(
    items: &[&'a str],
    visual_index: usize,
    search_query: &str,
) -> Option<&'a str> {
    let filtered_items = filter_items(items, search_query);

    if visual_index < filtered_items.len() { Some(filtered_items[visual_index]) } else { None }
}
