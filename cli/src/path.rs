use std::path::PathBuf;
use directories::ProjectDirs;

pub fn get_path(user_path: Option<PathBuf>) -> Result<PathBuf, std::io::Error> {
    if let Some(path) = user_path {
        return Ok(path);
    }
    
    let project_dirs = get_project_dirs().ok_or_else(|| std::io::ErrorKind::NotFound)?;
    if !project_dirs.data_dir().exists() {
        println!("DataDirectory doesn't exist. Generating it now.");
        std::fs::create_dir_all(project_dirs.data_dir())?;
    }
    let db_path = project_dirs.data_dir().join("library.sqlite3");
    Ok(db_path)
}

fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "lilo", "quicklearnCli")
}
