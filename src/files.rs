use std::{
    fs::{self, DirEntry},
    path::Path,
};

pub fn get_files(path: &Path) -> Vec<DirEntry> {
    get_dir_entries(path)
        .filter(|entry| entry.path().is_file())
        .collect()
}

pub fn get_dirs(path: &Path) -> Vec<DirEntry> {
    get_dir_entries(path)
        .filter(|entry| entry.path().is_dir())
        .collect()
}

fn get_dir_entries(path: &Path) -> impl Iterator<Item = DirEntry> {
    let name = get_name(path);

    fs::read_dir(path)
        .expect(&format!("Error reading directory: {name}"))
        .filter_map(Result::ok)
        .filter(|entry| !is_hidden(entry))
}

pub fn get_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|x| x.to_str())
        .map(|x| x.to_string())
        .expect("Could not extract path name")
}

fn is_hidden(entry: &DirEntry) -> bool {
    get_name(&entry.path()).starts_with(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_files_contents() {
        let test_dir = Path::new("./test_data");

        let mut file_contents: Vec<String> = get_files(test_dir)
            .iter()
            .map(|file| fs::read_to_string(file.path()).unwrap())
            .collect();

        assert_eq!(file_contents.len(), 3);

        let path = |name: &str| test_dir.join(Path::new(name));
        let mut expected_file_contents = vec![
            fs::read_to_string(path("file1")).unwrap(),
            fs::read_to_string(path("file2")).unwrap(),
            fs::read_to_string(path("file3")).unwrap(),
        ];

        file_contents.sort();
        expected_file_contents.sort();
        assert_eq!(file_contents, expected_file_contents);
    }

    #[test]
    fn test_get_dirs_names() {
        let test_dir = Path::new("./test_data");

        let mut dir_names: Vec<String> = get_dirs(test_dir)
            .iter()
            .map(|dir| get_name(&dir.path()))
            .collect();

        assert_eq!(dir_names.len(), 3);

        let mut expected_dir_names = vec!["dir1", "dir2", "dir3"];

        dir_names.sort();
        expected_dir_names.sort();
        assert_eq!(dir_names, expected_dir_names);
    }

    #[test]
    fn test_get_dirs_subfiles() {
        let test_dir = Path::new("./test_data");

        let mut child_file_contents: Vec<Vec<String>> = get_dirs(test_dir)
            .iter()
            .map(|dir| get_files(&dir.path()))
            .map(|files| {
                let mut contents: Vec<String> = files
                    .iter()
                    .map(|file| fs::read_to_string(file.path()).unwrap())
                    .collect();
                contents.sort();

                return contents;
            })
            .collect();

        assert_eq!(child_file_contents.len(), 3);
        assert!(child_file_contents.iter().all(|c| c.len() == 3));

        let path = |dir: &str, name: &str| test_dir.join(Path::new(dir)).join(Path::new(name));

        let mut contents1 = vec![
            fs::read_to_string(path("dir1", "dir1_file1")).unwrap(),
            fs::read_to_string(path("dir1", "dir1_file2")).unwrap(),
            fs::read_to_string(path("dir1", "dir1_file3")).unwrap(),
        ];
        contents1.sort();

        let mut contents2 = vec![
            fs::read_to_string(path("dir2", "dir2_file1")).unwrap(),
            fs::read_to_string(path("dir2", "dir2_file2")).unwrap(),
            fs::read_to_string(path("dir2", "dir2_file3")).unwrap(),
        ];
        contents2.sort();

        let mut contents3 = vec![
            fs::read_to_string(path("dir3", "dir3_file1")).unwrap(),
            fs::read_to_string(path("dir3", "dir3_file2")).unwrap(),
            fs::read_to_string(path("dir3", "dir3_file3")).unwrap(),
        ];
        contents3.sort();

        let mut expected_child_file_contents = vec![contents1, contents2, contents3];

        child_file_contents.sort();
        expected_child_file_contents.sort();
        assert_eq!(child_file_contents, expected_child_file_contents);
    }

    #[test]
    fn test_get_dir_entries_names() {
        let test_dir = Path::new("./test_data");

        let mut entry_names: Vec<String> = get_dir_entries(test_dir)
            .map(|dir| get_name(&dir.path()))
            .collect();

        assert_eq!(entry_names.len(), 6);

        let mut expected_entry_names = vec!["dir1", "dir2", "dir3", "file1", "file2", "file3"];

        entry_names.sort();
        expected_entry_names.sort();
        assert_eq!(entry_names, expected_entry_names);
    }

    #[test]
    #[should_panic]
    fn test_get_dir_entries_invalid() {
        let _: Vec<_> = get_dir_entries(Path::new("./test_datas")).collect();
    }

    #[test]
    fn test_is_hidden_names() {
        let test_dir = "./test_data";

        let mut hidden_entry_names: Vec<String> = fs::read_dir(test_dir)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| is_hidden(entry))
            .map(|entry| get_name(&entry.path()))
            .collect();

        let mut expected_hidden_entry_names = vec![".hidden_dir", ".hidden_file"];

        hidden_entry_names.sort();
        expected_hidden_entry_names.sort();
        assert_eq!(hidden_entry_names, expected_hidden_entry_names);
    }

    #[test]
    fn test_get_name() {
        let path = Path::new("./test_data");
        let name = get_name(path);
        assert_eq!(name, String::from("test_data"));

        let path = Path::new("./test_data/file1.txt");
        let name = get_name(path);
        assert_eq!(name, String::from("file1"));
    }
}
