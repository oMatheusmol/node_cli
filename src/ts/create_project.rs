#[warn(unused_variables)]
use std::{fs, io::Write};

pub fn create_project(project_name: &str, language: &str) {
    create_project_founder(project_name).unwrap();
    create_main_file(project_name, language, "main").unwrap();
    create_json_file(project_name).unwrap();
    create_env_file(project_name).unwrap();
    create_gitignore_file(project_name).unwrap();
    create_tsconfig_file(project_name).unwrap();
    create_jest_config_file(project_name).unwrap();
    create_main_file_test(project_name).unwrap();
    create_prettier_file(project_name).unwrap();
    install_dependencies(project_name).unwrap();
    start_git(project_name).unwrap();
    format_files(project_name).unwrap();
    open_vscode(project_name).unwrap();
}

fn create_prettier_file(project_name: &str) -> std::io::Result<()> {
    let mut prettier = fs::File::create(format!("{}/.prettierrc.js", project_name))?;
    prettier.write_all(
        "module.exports = {
            singleQuote: true,
            tabWidth: 2,
        };
          "
        .replace("'", "\"")
        .as_bytes(),
    )?;
    Ok(())
}

fn format_files(project_name: &str) -> std::io::Result<()> {
    let mut prettier = std::process::Command::new("npm");
    prettier.arg("run").arg("format").current_dir(project_name);
    prettier.spawn()?.wait()?;
    Ok(())
}

fn open_vscode(project_name: &str) -> std::io::Result<()> {
    let mut vscode = std::process::Command::new("code");
    vscode.arg(".").current_dir(project_name);
    vscode.spawn()?.wait()?;
    Ok(())
}

fn start_git(project_name: &str) -> std::io::Result<()> {
    let mut git = std::process::Command::new("git");
    git.arg("init").current_dir(project_name);
    git.spawn()?.wait()?;

    let mut add = std::process::Command::new("git");
    add.arg("add").arg(".").current_dir(project_name);
    add.spawn()?.wait()?;
    Ok(())
}

fn create_main_file_test(project_name: &str) -> std::io::Result<()> {
    let mut main_test = fs::File::create(format!("{}/src/main.test.ts", project_name))?;
    main_test.write_all(
        "import { describe, it, expect } from '@jest/globals';
import { Main } from './main';

describe('Main', () => {
    it('should work', () => {
    expect(new Main().execute()).toBe(1);
    });
});
    "
        .as_bytes(),
    )?;
    Ok(())
}

fn create_jest_config_file(project_name: &str) -> std::io::Result<()> {
    let mut jest_config = fs::File::create(format!("{}/jest.config.js", project_name))?;
    jest_config.write_all(
        "module.exports = {
    preset: 'ts-jest',
    testEnvironment: 'node',
    testMatch: ['**/*.test.ts'],
    moduleNameMapper: {
        '^@/(.*)$': '<rootDir>/src/$1',
    },
    globals: {
        transform: {
        '^.+.ts?$': ['ts-jest', { tsconfig: './tsconfig.json' }],
        },
    },
    };
          "
        .as_bytes(),
    )?;
    Ok(())
}

fn create_tsconfig_file(project_name: &str) -> std::io::Result<()> {
    let mut tsconfig = fs::File::create(format!("{}/tsconfig.json", project_name))?;
    tsconfig.write_all(
        "{
    'compilerOptions': {
        'target': 'es6',
        'module': 'commonjs',
        'strict': true,
        'esModuleInterop': true,
        'outDir': './dist',
        'sourceMap': true
    },
    'include': ['src/**/*.ts'],
    'exclude': ['node_modules', '**/*.test.ts']
}
      "
        .replace("'", "\"")
        .as_bytes(),
    )?;
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
        .arg("prettier")
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

fn create_gitignore_file(project_name: &str) -> std::io::Result<()> {
    let mut gitignore = fs::File::create(format!("{}/.gitignore", project_name))?;
    gitignore.write_all(
        "node_modules
dist/
.env.test
.env"
            .as_bytes(),
    )?;
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

fn create_main_file(project_name: &str, language: &str, file_name: &str) -> std::io::Result<()> {
    let src_dir = format!("{}/src", project_name);
    if !fs::metadata(&src_dir).is_ok() {
        fs::create_dir(&src_dir)?;
    }
    let mut file = fs::File::create(format!(
        "{}/src/{}.{}",
        &project_name, &file_name, &language
    ))?;
    let buffer = b"export class Main {
    constructor() {
        console.log('Hello World!');
    }
    
    execute(): number {
        return 1;
    }
}
      ";
    file.write_all(buffer)?;
    Ok(())
}

fn create_json_file(project_name: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(format!("{}/package.json", project_name))?;
    let new_string = "{
        'name': 'PROJECT_NAME',
        'version': '1.0.0',
        'description': 'project description',
        'main': 'src/main.ts',
        'author': 'Matheus Mol',
        'scripts': {
          'format': 'prettier --write . --loglevel silent',
          'ts-node': 'ts-node',
          'tsc': 'tsc',
          'tsc:check': 'npm run tsc -- --noEmit',
          'build': 'tsc -b ./tsconfig.json',
          'build:w': 'tsc -b ./tsconfig.json -w',
          'clean:tsc': 'rm tsconfig.tsbuildinfo',
          'test': 'jest',
          'test:cov': 'npm run test -- --coverage'
        },
        'keywords': [],
        'license': 'ISC',
        'dependencies': {}
      }
      "
    .replace("PROJECT_NAME", project_name)
    .replace("'", "\"");
    file.write_all(new_string.as_bytes())?;
    Ok(())
}

fn create_project_founder(project_name: &str) -> std::io::Result<()> {
    if !fs::metadata(project_name).is_ok() {
        fs::create_dir(project_name)?;
    }
    Ok(())
}
