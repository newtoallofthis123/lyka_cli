pub fn print_splash_screen(){
    bunt::println!("{$blue}+-+-+-+-+-+{/$}");
    bunt::println!("{$green}|🐶|{/$}{$white}L|y|k|a|{/$}");
    bunt::println!("{$yellow}+-+-+-+-+-+{/$}");
}

pub fn show_options(options: Vec<&str>) -> &str {
    let result = inquire::Select::new("Select an option", options).with_page_size(10).prompt().unwrap();
    result
}

pub fn get_conformation(msg: &str) -> bool {
    let result = inquire::Confirm::new(msg).with_default(true).prompt().unwrap();
    result
}

pub fn get_editor(msg:String, file_ext: String, cmd: String)->String{
    let content = inquire::Editor::new(msg.as_str()).with_file_extension(file_ext.as_str()).with_editor_command(std::ffi::OsStr::new(cmd.as_str())).prompt().unwrap();
    content
}

pub fn spinner()-> indicatif::ProgressBar{
    let pb = indicatif::ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::from_millis(120));
    pb.set_style(
        indicatif::ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "⣾",
                "⣽",
                "⣻",
                "⢿",
                "⡿",
                "⣟",
                "⣯",
                "⣷",
                "(200 Ok)"
            ]),
    );
    pb.set_message("Sending Request...");
    pb
}

pub fn print_footer(){
    bunt::println!("With {$red}❤️{/$}{$white} from Lkya{/$}");
}