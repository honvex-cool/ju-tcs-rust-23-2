use std::fs;
use std::path::Path;

pub fn head(path: &Path, n: usize) -> Vec<String> {
    read_file_lines(path).into_iter().take(n).collect()
}

pub fn tail(path: &Path, n: usize) -> Vec<String> {
    let lines = read_file_lines(path);
    if lines.len() <= n {
        lines
    } else {
        let to_skip = lines.len() - n;
        lines.into_iter().skip(to_skip).collect()
    }
}

fn read_file_lines(path: &Path) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_works_with_small_n() {
        let lines = head(&Path::new("test_files/lines.txt"), 2);
        assert_eq!(lines, vec!["a", "b"]);
    }

    #[test]
    fn head_works_with_big_n() {
        let lines = head(&Path::new("test_files/lines.txt"), 10);
        assert_eq!(lines, vec!["a", "b", "", "c", "d", "e"]);
    }

    #[test]
    fn tail_works_with_small_n() {
        let lines = tail(&Path::new("test_files/lines.txt"), 2);
        assert_eq!(lines, vec!["d", "e"]);
    }

    #[test]
    fn tail_works_with_big_n() {
        let lines = tail(&Path::new("test_files/lines.txt"), 10);
        assert_eq!(lines, vec!["a", "b", "", "c", "d", "e"]);
    }
}
