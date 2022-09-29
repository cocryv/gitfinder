use clap::Parser;
use reqwest::header::USER_AGENT;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author="Valentin Cocry", version="1.0", about="Fast and lightweight git searching tools", long_about = None)]
struct Args {

    /// Repository owner username
    #[clap(short, long, value_parser)]
    user: Option<String>,

    /// Name of the repository
    #[clap(short, long, value_parser)]
    name: Option<String>,

    /// Programming language of the repository
    #[clap(short, long, value_parser)]
    language: Option<String>,
}

#[tokio::main]
async fn main() {
   let args = Args::parse();

   let mut user : &String = &String::from("");
   let mut name : &String = &String::from("");
   let mut language : &String = &String::from("");

    if let Some(value) = &args.user {
        user = value;
    }
    if let Some(value) = &args.name {
        name = value;
    }
    if let Some(value) = &args.language {
        language = value;
    }

    let mut is_runnable = false;

//    let config = Config::build(args).unwrap();
    if user != "" {
        println!("Utilisateur recherché : {}", user);
        is_runnable = true;
    }
    if name != "" {
        println!("Nom de repository recherché : {}", name);
        is_runnable = true;
    }
    if language != "" {
        println!("Language recherché : {}", language);
    }

    if !is_runnable{
        panic!("You need at least user or name args !")
    }

    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/search/repositories?q=repo:cocryv/{}&order=desc&per_page=30",name);
    println!("{url}");
    let response = client
        .get(url)
        .header(USER_AGENT, "My Rust Program 1.0")
        .send()
        .await
        .unwrap();
    println!("{}",response.text().await.unwrap())

}