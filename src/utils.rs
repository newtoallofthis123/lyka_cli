use std::{process::Command, io::Write};

use clipboard::{ClipboardContext, ClipboardProvider};

use crate::types::Code;

pub fn get_editor() -> Vec<&'static str>{
    // List of text editors to check in order of preference
    let editors = ["micro", "vim", "nano", "emacs", "subl"];

    let mut installed_editors: Vec<&str> = Vec::new();

    // Determine the appropriate command for checking executables based on the OS
    let check_command = if cfg!(windows) { "where" } else { "which" };

    for editor in &editors {
        let output = Command::new(check_command)
            .arg(editor)
            .output()
            .expect("Failed to execute command.");

        if output.status.success() {
            installed_editors.push(editor);
        }
    }

    return installed_editors;
}

pub fn if_file_exists(filename: &str)->bool{
    let file = std::path::Path::new(filename);
    let result = file.exists();
    return result;
}

pub fn get_editor_content(file_ext: String) ->String{
    let editors = get_editor();
    let editor = crate::cli::show_options(editors);
    let content = crate::cli::get_editor("📝 Enter your Code".to_string(), file_ext, editor.to_string());
    content
}

pub fn get_ext_from_lang(ext:&str)->String{
    match ext {
        "c" => ".c".to_string(),
        "cpp" => ".cpp".to_string(),
        "java" => ".java".to_string(),
        "python" => ".py".to_string(),
        "rust" => ".rs".to_string(),
        "go" => ".go".to_string(),
        "ruby" => ".rb".to_string(),
        "html" => ".html".to_string(),
        "css" => ".css".to_string(),
        "toml" => ".toml".to_string(),
        "json" => ".json".to_string(),
        "text" => ".txt".to_string(),
        "markdown" => ".md".to_string(),
        "jsx" => ".jsx".to_string(),
        "tsx" => ".tsx".to_string(),
        "typescript" => ".ts".to_string(),
        "javascript" => ".js".to_string(),
        _=>".txt".to_string()
    }
}

pub fn try_to_get_name()->String{
    //get username from git config
    let output = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .output()
        .expect("Failed to execute command.");

    let username = String::from_utf8(output.stdout).unwrap();

    if username.trim().is_empty(){
        //get username from system
        let output = Command::new("whoami")
            .output()
            .expect("Failed to execute command.");

        let username = String::from_utf8(output.stdout).unwrap();
        return username;
    } else {
        return username.trim_end().to_string();
    }
}

//* Copy the shortened URL to the clipboard */
pub fn copy(msg: &str){
    let mut board: ClipboardContext = ClipboardProvider::new().unwrap();
    board.set_contents(msg.to_owned()).unwrap();
}

pub fn save_to_file(code: Code){
    let prohibited_chars = ['/', '\\', '?', '%', '*', ':', '|', '"', '<', '>', '.'];
    let mut filename = code.title.clone();
    filename = filename.to_lowercase().replace(" ", "_");
    //remove all chars after the last dot
    let mut filename = filename.split(".").collect::<Vec<&str>>()[0].to_string();
    for c in prohibited_chars.iter(){
        filename = filename.replace(*c, "");
    }
    let filename = filename + get_ext_from_lang(code.lang.as_str()).as_str() + ".txt";
    let mut file = std::fs::File::create(filename.clone()).unwrap();
    file.write_all(code.content.as_bytes()).unwrap();
    bunt::println!("Saved to file: {$green}{}{/$}", filename);
}