use alloc::string::{String, ToString};

pub(crate) fn is_package_relative_path(path: &str) -> bool {
    !path.is_empty()
        && path.ends_with(']')
        && find_matching_opening_delimiter(path, path.len() - 1).is_some()
}

/// Given a string `path, find the index range of the outermost opening and closing
/// delimiters `[` and `]`.
pub(crate) fn split_package_relative_path_outer(path: &str) -> Option<(String, String)> {
    let outermost_closing_index = find_outermost_closing_delimiter(path)?;
    let outermost_opening_index = find_matching_opening_delimiter(path, path.len() - 1)?;

    Some((
        unescape_delimiters(&path[..outermost_opening_index]),
        path[outermost_opening_index + 1..outermost_closing_index].to_string(),
    ))
}

// Unescape delimiters in the given path to give clients the 'real' path
// when extracting paths from the packaged part of a package-relative path.
//
// If path is a package-relative path, we assume the packaged portion of
// that path has already been escaped and only process the package portion.
pub(crate) fn unescape_delimiters(path: &str) -> String {
    if path.is_empty() {
        return path.to_string();
    }

    let escape_range_begin = 0;
    let escape_range_end = if path.ends_with(']') {
        path.len()
    } else if let Some(outer_open_index) = find_matching_opening_delimiter(path, path.len() - 1) {
        outer_open_index
    } else {
        path.len()
    };

    let escaped_string = &path[escape_range_begin..escape_range_end];
    let escaped_string = escaped_string.replace("\\[", "[");
    let escaped_string = escaped_string.replace("\\]", "]");

    escaped_string + &path[escape_range_end..path.len()]
}

/// Given an index `closing_delimiter_index` for `path` pointing to a closing
/// `]` character, returns the [`Some`] index to the corresponding opening `[`
/// character, or [`None`] if one can't be found.
pub(crate) fn find_matching_opening_delimiter(
    path: &str,
    closing_delimiter_index: usize,
) -> Option<usize> {
    let mut num_open_needed = 1;
    let mut char_indices = path.char_indices().rev().peekable();

    while let Some((i, ch)) = char_indices.next() {
        if i >= closing_delimiter_index {
            continue;
        }

        if ch == '[' || ch == ']' {
            // Ignore this delimiter if it's been escaped.
            if let Some((_, '\\')) = char_indices.peek() {
                char_indices.next();
                continue;
            }

            if ch == '[' {
                num_open_needed -= 1;
            } else {
                num_open_needed += 1;
            }
        }

        if num_open_needed == 0 {
            return Some(i);
        }
    }

    None
}

pub(crate) fn find_outermost_closing_delimiter(path: &str) -> Option<usize> {
    if path.is_empty() || !path.ends_with(']') {
        return None;
    }
    Some(path.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape_delimiters() {
        let path = "/dir/\\[foo\\].package[bar.package[baz.file]]";
        assert_eq!(
            unescape_delimiters(path),
            "/dir/[foo].package[bar.package[baz.file]]"
        );
    }

    #[test]
    fn test_find_matching_opening_delimiter() {
        assert_eq!(find_matching_opening_delimiter("[asd]", 4), Some(0));
        assert_eq!(find_matching_opening_delimiter("a[sd]", 4), Some(1));
        assert_eq!(find_matching_opening_delimiter("a[s\\[d]", 6), Some(1));
    }

    #[test]
    fn test_is_package_relative_path() {
        assert!(is_package_relative_path("[asd]"));
        assert!(is_package_relative_path("a[sd]"));
        assert!(is_package_relative_path("[a\\[sd]"));
        assert!(!is_package_relative_path("a\\[sd\\]"));
        assert!(!is_package_relative_path("asd"));
    }

    #[test]
    fn test_split_package_relative_path_outer() {
        let path = "/dir/\\[foo\\].package[bar.package[baz.file]]";
        let paths = split_package_relative_path_outer(path);
        assert_eq!(
            paths,
            Some((
                "/dir/[foo].package".to_string(),
                "bar.package[baz.file]".to_string()
            ))
        );
    }
}
