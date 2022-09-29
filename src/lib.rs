// use std::error::Error;

// pub struct Config<'a> {
//     pub user: &'a String,
//     pub name: &'a String,
//     pub langage: &'a String,
// }

// impl<'a> Config<'a> {
//     pub fn build(args: &[String]) -> Result<Config,&'static str> {

//         if args.len() < 2 {
//             return Err("not enough argument");
//         }

//         let name = &args[1];
//         let user = &args[1];
//         let langage = &args[1];
        

//         Ok(Config { name:&name,user:&user,langage:&langage })

//     }
// }

// pub fn run(config: Config)-> Result<(),Box<dyn Error>>{
//     // search(&config.name);
//     Ok(())
// }

// pub fn search<'a>(name: &'a str) -> Vec<&'a str>{
//     return ();
// }