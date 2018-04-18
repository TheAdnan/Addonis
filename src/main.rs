use std::error::Error;
use std::fs::File;
use std::fs::create_dir;
use std::path::Path;
use std::io::{stdin,stdout,Write};

fn main() {
    //Create the manifest.json file
    let mut name = String::new();
    print!("Enter the name of the add-on: ");
    let _=stdout().flush();

    stdin().read_line(&mut name).expect("Did not enter a correct string");

    if let Some('\n')=name.chars().next_back() {
        name.pop();
    }
    if let Some('\r')=name.chars().next_back() {
        name.pop();
    }

    let mut description = String::new();
    print!("Enter a description for the add-on: ");
    let _ = stdout().flush();

    stdin().read_line(&mut description).expect("Did not enter a correct string");

    if let Some('\n')=description.chars().next_back() {
        description.pop();
    }
    if let Some('\r')=description.chars().next_back() {
        description.pop();
    }

//    let mut permissions = String::new();
//    print!("Enter add-ons permissions (separated with commas): ");
//    let _ = stdout().flush();
//
//    stdin().read_line(&mut permissions).expect("Did not enter a correct string");
//
//    if let Some('\n')=permissions.chars().next_back() {
//        permissions.pop();
//    }
//    if let Some('\r')=permissions.chars().next_back() {
//        permissions.pop();
//    }

    let mut manifest_json = "{\n\"manifest_version\": 2,\n\"name\": \"".to_string();
    manifest_json.push_str(&name);
    manifest_json.push_str(&"\",\n\"version\": \"1.0\",\n\"description\":\"".to_string());
    manifest_json.push_str(&description);
    manifest_json.push_str(&"\",\n\n\"icons\":{\n\"48\":\"icons/icon.png\"\n}".to_string());
    manifest_json.push_str(&"\",\n\n\"browser_action\":{\n\"default_icon\":\"icons/icon.png\"\n}".to_string());
    manifest_json.push_str(&"\",\n\n\"background\":{\n\"scripts\":[\"index.js\"]\n}\n}".to_string());


    let mut indexjs_entry = "$ = (queryString) => document.querySelector(queryString);\n//Shortcut for jQuery-like syntax in pure JS".to_string();

    create_dir(&name);
    create_dir(&format!("{}/{}", &name, "icons".to_string()));
    let mut manifest_json_path = String::new();
    manifest_json_path.push_str(&name);
    manifest_json_path.push_str(&"/manifest.json".to_string());

    let mut index_js_path = String::new();
    index_js_path.push_str(&name);
    index_js_path.push_str(&"/index.js".to_string());


    let path = Path::new(&manifest_json_path);
    let display = path.display();

    let indexjs = Path::new(&index_js_path);


    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    let mut indexjs_file = match File::create(&indexjs) {
        Err(why) => panic!("Couldn't create {}: {}",
                           indexjs.display(),
                           why.description()),
        Ok(indexjs_file) => indexjs_file,
    };


    match indexjs_file.write_all(indexjs_entry.as_bytes()) {
        Err(why) => {
            panic!("Couldn't write to {}: {}", display,
                   why.description())
        },
        Ok(_) => println!("Index.js created!"),
    }

    match file.write_all(manifest_json.as_bytes()) {
        Err(why) => {
            panic!("Couldn't write to {}: {}", display,
                   why.description())
        },
        Ok(_) => println!("Add-on skeleton was successfully created!"),
    }
}