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

        let mut expected_file_contents =
            vec!["file1 content\n", "file2 content\n", "file3 content\n"];

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

        assert_eq!(dir_names.len(), 4);

        let mut expected_dir_names = vec!["BTN vs BB", "CO vs BB", "HJ vs BB", "LJ vs BB"];

        dir_names.sort();
        expected_dir_names.sort();
        assert_eq!(dir_names, expected_dir_names);
    }

    #[test]
    fn test_get_dirs_subfiles() {
        let test_dir = Path::new("./test_data/BTN vs BB");

        let mut file_contents: Vec<String> = get_dirs(test_dir)
            .iter()
            .map(|dir| {
                let files = get_files(&dir.path());
                assert_eq!(files.len(), 1);
                let file = &files[0];

                let filename = get_name(&file.path());
                assert_eq!(filename, "after_check");

                fs::read_to_string(&file.path()).unwrap()
            })
            .collect();

        let expected_content = "Tree	Equity(*)	EV	Bet 82.5	Check
8s8d8c	42.562	41.461	77.582	24.42	
8s8d6d	61.060	13.549	32.860	78.143	
Ks7d4c	33.603	34.212	24.302	61.695	
As5s5d	77.418	34.440	6.024	88.995	
As7d4c	49.40	55.912	20.64	99.341	
Ts6s4d	70.407	10.865	3.348	77.653	
";
        let mut expected_contents: Vec<String> =
            (0..4).map(|_| expected_content.to_string()).collect();

        expected_contents.sort();
        file_contents.sort();

        assert_eq!(file_contents, expected_contents);
    }

    #[test]
    fn test_get_dir_entries_names() {
        let test_dir = Path::new("./test_data");

        let mut entry_names: Vec<String> = get_dir_entries(test_dir)
            .map(|dir| get_name(&dir.path()))
            .collect();

        assert_eq!(entry_names.len(), 7);

        let mut expected_entry_names = vec![
            "BTN vs BB",
            "CO vs BB",
            "HJ vs BB",
            "LJ vs BB",
            "file1",
            "file2",
            "file3",
        ];

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
