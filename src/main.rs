use std::{
    fs,
    io::{Read, Write},
    path,
};

fn main() {
    let project_name = "my-app";
    let language = "ts";

    create_project_founder(project_name).unwrap();

    let file_main_content = read_language_file(language, "main");
    create_main_file(project_name, language, "main", &file_main_content).unwrap();

    let file_package_json_content = read_language_file(language, "package");
    create_json_file(&file_package_json_content, project_name).unwrap();

    create_env_file(project_name).unwrap();

    let file_gitignore_content = read_language_file(language, ".gitignore");
    create_gitignore_file(project_name, &file_gitignore_content).unwrap();

    let file_tsconfig_content = read_language_file(language, "tsconfig");
    create_tsconfig_file(project_name, file_tsconfig_content).unwrap();

    let file_jest_config_content = read_language_file(language, "jest.config");
    create_jest_config_file(project_name, file_jest_config_content).unwrap();

    let file_main_test_content = read_language_file(language, "main.test");
    create_main_file_test(project_name, file_main_test_content).unwrap();

    install_dependencies(project_name).unwrap();

    start_git(project_name).unwrap();
}

fn start_git(project_name: &str) -> std::io::Result<()> {
    let mut git = std::process::Command::new("git");
    git.arg("init").current_dir(project_name);
    git.spawn()?.wait()?;
    Ok(())
}

fn create_main_file_test(
    project_name: &str,
    file_main_test_content: String,
) -> std::io::Result<()> {
    let mut main_test = fs::File::create(format!("{}/src/main.test.ts", project_name))?;
    main_test.write_all(file_main_test_content.as_bytes())?;
    Ok(())
}

fn create_jest_config_file(
    project_name: &str,
    file_jest_config_content: String,
) -> std::io::Result<()> {
    let mut jest_config = fs::File::create(format!("{}/jest.config.js", project_name))?;
    jest_config.write_all(file_jest_config_content.as_bytes())?;
    Ok(())
}

fn create_tsconfig_file(project_name: &str, file_tsconfig_content: String) -> std::io::Result<()> {
    let mut tsconfig = fs::File::create(format!("{}/tsconfig.json", project_name))?;
    tsconfig.write_all(file_tsconfig_content.as_bytes())?;
    Ok(())
}

fn install_dependencies(project_name: &str) -> std::io::Result<()> {
    let mut dev_dependencies = std::process::Command::new("npm");
    dev_dependencies
        .arg("install")
        .arg("-D")
        .arg("@types/jest")
        .arg("@types/node")
        .arg("jest")
        .arg("ts-node")
        .arg("ts-jest")
        .arg("typescript")
        .current_dir(project_name);
    dev_dependencies.spawn()?.wait()?;

    let mut dependencies = std::process::Command::new("npm");
    dependencies
        .arg("install")
        .arg("dotenv")
        .current_dir(project_name);
    dependencies.spawn()?.wait()?;
    Ok(())
}

fn create_gitignore_file(project_name: &str, file_gitignore_content: &str) -> std::io::Result<()> {
    let mut gitignore = fs::File::create(format!("{}/.gitignore", project_name))?;
    gitignore.write_all(file_gitignore_content.as_bytes())?;
    Ok(())
}

fn create_env_file(project_name: &str) -> std::io::Result<()> {
    let mut env = fs::File::create(format!("{}/.env", project_name))?;
    let mut env_example = fs::File::create(format!("{}/.env.example", project_name))?;
    let mut env_test = fs::File::create(format!("{}/.env.test", project_name))?;
    let mut env_test_example = fs::File::create(format!("{}/.env.test.example", project_name))?;
    env.write_all("".as_bytes())?;
    env_example.write_all("".as_bytes())?;
    env_test.write_all("".as_bytes())?;
    env_test_example.write_all("".as_bytes())?;
    Ok(())
}

fn create_main_file(
    project_name: &str,
    language: &str,
    file_name: &str,
    content: &String,
) -> std::io::Result<()> {
    let src_dir = format!("{}/src", project_name);
    if !fs::metadata(&src_dir).is_ok() {
        fs::create_dir(&src_dir)?;
    }
    let mut file = fs::File::create(format!(
        "{}/src/{}.{}",
        &project_name, &file_name, &language
    ))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn create_json_file(content: &str, project_name: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(format!("{}/package.json", project_name))?;
    let new_string = content.replace("PROJECT_NAME", project_name);
    file.write_all(new_string.as_bytes())?;
    Ok(())
}

fn read_language_file(language: &str, file_name: &str) -> String {
    let path_string = format!("src/{}/{}.txt", language, file_name);
    let my_path = path::Path::new(&path_string);
    let mut file = fs::File::open(my_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
}

fn create_project_founder(project_name: &str) -> std::io::Result<()> {
    if !fs::metadata(project_name).is_ok() {
        fs::create_dir(project_name)?;
    }
    Ok(())
}
