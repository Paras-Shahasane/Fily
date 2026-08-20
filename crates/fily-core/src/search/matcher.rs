/// Checks whether a filename matches a search query.
///
/// Matching is substring-based rather than exact matching.
///
/// Examples:
///
/// `report` matches `report.pdf`
/// `report` matches `old-report.txt`
/// `Report` matches `report.pdf` when case-insensitive matching is enabled.
pub fn matches_name(
    name: &str,
    query: &str,
    case_insensitive: bool,
) -> bool {
    if case_insensitive {
        name.to_lowercase().contains(&query.to_lowercase())
    } else {
        name.contains(query)
    }
}