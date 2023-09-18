use types::AddCode;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="Lyka", author="Ishan Joshi", version, about="The Pastebin for dogs", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=false, short, long, help="The title of the code snippet")]
    title: Option<String>,

    #[arg(required=false, short, long, help="The author of the code snippet")]
    author: Option<String>,

    #[arg(required=false, short, long, help="The language of the code snippet")]
    lang: Option<String>,

    #[arg(required=false, short, long, help="The file with the code")]
    file: Option<String>,
}


mod types;
mod cli;
mod web;
mod utils;

#[tokio::main]
async fn main() {
    cli::print_splash_screen();

    let args = Args::parse();

    let mut code_to_add = AddCode{
        title: String::new(),
        author: String::new(),
        content: String::new(),
        lang: String::new()
    };

    if args.title.is_some(){
        bunt::println!("📝 Title: {}", args.title.clone().unwrap());
        code_to_add.title = args.title.unwrap();
    } else{
        code_to_add.title = inquire::Text::new("😎 Paste Title:").with_default(&args.file.clone().unwrap_or("Untitled".to_string())).prompt().unwrap();
    }

    if args.author.is_some(){
        bunt::println!("👤 Author: {}", args.author.clone().unwrap());
        code_to_add.author = args.author.unwrap();
    } else{
        code_to_add.author = inquire::Text::new("🤔 Your Name Please:").with_default(&utils::try_to_get_name()).prompt().unwrap();
    }

    if args.lang.is_some(){
        bunt::println!("🌐 Language: {}", args.lang.clone().unwrap());
        code_to_add.lang = args.lang.unwrap();
    } else{
        code_to_add.lang = cli::show_options(vec!["python", "c", "c++", "java", "javascript", "rust", "go", "ruby", "html", "csss", "text"]).to_string();
    }

    if args.file.is_none(){
        code_to_add.content = utils::get_editor_content(utils::get_ext_from_lang(code_to_add.lang.as_str()));
    } else{
        if utils::if_file_exists(args.file.clone().unwrap().as_str()){
            bunt::println!("📁 Read File: {}", args.file.clone().unwrap());
            code_to_add.content = std::fs::read_to_string(args.file.clone().unwrap()).unwrap();
        } else{
            bunt::println!("🚫 File not found, enter the content");
            code_to_add.content = utils::get_editor_content(utils::get_ext_from_lang(code_to_add.lang.as_str()));
        }
    }

    bunt::println!("Got The {$yellow}Struct{/$} :)");

    let confirmation = cli::get_conformation("✨ Do you want to continue?");
    if !confirmation{
        bunt::println!("👋 Bye!");
        std::process::exit(0);
    }
    let hash = web::add_code(code_to_add).await;

    bunt::println!("🔗 Link: {$underline}https://noobscience.rocks/code/{}{/$}", hash);
    utils::copy(format!("https://noobscience.rocks/code/{}", hash).as_str());
    bunt::println!("Coped to clipboard!")
}