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
    create_docker_file(project_name).unwrap();
    create_docker_compose_file(project_name).unwrap();
    create_docker_compose_override_file(project_name).unwrap();
    create_dot_dockercontainer_folder(project_name).unwrap();
    create_devcontainer_json_file(project_name).unwrap();
    create_docker_compose_yml(project_name).unwrap();
    create_dot_docker_folder(project_name).unwrap();
    create_start_sh_file(project_name).unwrap();
    create_gitkeep_file(project_name).unwrap();
    create_p10k_file(project_name).unwrap();
    install_dependencies(project_name).unwrap();
    format_files(project_name).unwrap();
    start_git(project_name).unwrap();
    open_vscode(project_name).unwrap();
}

fn create_gitkeep_file(project_name: &str) -> std::io::Result<()> {
    let mut gitkeep = fs::File::create(format!("{}/.docker/zsh/history/.gitkeep", project_name))?;
    gitkeep.write_all("".as_bytes()).unwrap();
    Ok(())
}

fn create_start_sh_file(project_name: &str) -> std::io::Result<()> {
    let mut start_sh = fs::File::create(format!("{}/.docker/start.sh", project_name))?;
    start_sh
        .write_all(
            "#!/bin/bash
    
npm install

tail -f /dev/null

    "
            .as_bytes(),
        )
        .unwrap();
    Ok(())
}

fn create_dot_docker_folder(project_name: &str) -> std::io::Result<()> {
    fs::create_dir(format!("{}/.docker", project_name))?;
    fs::create_dir(format!("{}/.docker/zsh", project_name))?;
    fs::create_dir(format!("{}/.docker/zsh/history", project_name))?;
    fs::create_dir(format!("{}/.docker/zsh/powerlevel10k", project_name))?;
    Ok(())
}

fn create_docker_compose_yml(project_name: &str) -> std::io::Result<()> {
    let mut docker_compose = fs::File::create(format!(
        "{}/.devcontainer/docker-compose.yml",
        project_name
    ))?;
    docker_compose
        .write_all("version: '3'
services:
    app:  
        volumes:
            - ..:/workspaces:cached
        command: /bin/sh -c 'while sleep 1000; do :; done'
        ".replace("'", "\"").as_bytes()).unwrap();
    Ok(())
}

fn create_devcontainer_json_file(project_name: &str) -> std::io::Result<()> {
    let mut devcontainer =
        fs::File::create(format!("{}/.devcontainer/devcontainer.json", project_name))?;
    devcontainer.write_all("{
        'name': 'TypeScript App',
        'dockerComposeFile': [
            '../docker-compose.yml',
            '../docker-compose.override.yml',
            'docker-compose.yml'
        ],
        'service': 'app',
        'settings': {
            'terminal.integrated.defaultProfile.linux': 'zsh'
          },
        'workspaceFolder': '/home/node/app',
        'extensions': ['firsttris.vscode-jest-runner']
    }
    ".replace("'", "\"").as_bytes()).unwrap();
    Ok(())
}

fn create_dot_dockercontainer_folder(project_name: &str) -> std::io::Result<()> {
    fs::create_dir(format!("{}/.devcontainer", project_name))?;
    Ok(())
}

fn create_docker_compose_override_file(project_name: &str) -> std::io::Result<()> {
    let mut docker_compose_override =
        fs::File::create(format!("{}/docker-compose.override.yml", project_name))?;
    docker_compose_override
        .write_all(
            "version: '3'

services:
  app:
    volumes:
      - ./.docker/zsh/powerlevel10k/.p10k.zsh:/home/node/.p10k.zsh:delegated
      - ./.docker/zsh/history:/home/node/zsh:delegated"
                .as_bytes(),
        )
        .unwrap();
    Ok(())
}

fn create_docker_compose_file(project_name: &str) -> std::io::Result<()> {
    let mut docker_compose = fs::File::create(format!("{}/docker-compose.yml", project_name))?;
    docker_compose
        .write_all(
            "version: '3'

services:
    app:
        build: .
        container_name: PROJECT_NAME
        volumes:
            - .:/home/node/app
          "
            .replace("PROJECT_NAME", project_name)
            .as_bytes(),
        )
        .unwrap();

    Ok(())
}

fn create_docker_file(project_name: &str) -> std::io::Result<()> {
    let mut docker = fs::File::create(format!("{}/Dockerfile", project_name))?;
    docker.write_all("FROM node:16.19.1-slim

RUN mkdir -p /usr/share/man/man1 && \
    echo 'deb http://ftp.debian.org/debian stretch-backports main' | tee /etc/apt/sources.list.d/stretch-backports.list && \
    apt update && apt install -y \
    git \
    ca-certificates \
    openjdk-11-jre \
    zsh \
    curl \
    wget \
    fonts-powerline \
    procps

RUN npm install -g @nestjs/cli@8.2.5 npm@8.5.5

ENV JAVA_HOME=\"/usr/lib/jvm/java-11-openjdk-amd64\"

WORKDIR /home/node/app

RUN sh -c \"$(wget -O- https://github.com/deluan/zsh-in-docker/releases/download/v1.1.2/zsh-in-docker.sh)\" -- \
    -t https://github.com/romkatv/powerlevel10k \
    -p git \
    -p git-flow \
    -p https://github.com/zdharma-continuum/fast-syntax-highlighting \
    -p https://github.com/zsh-users/zsh-autosuggestions \
    -p https://github.com/zsh-users/zsh-completions \
    -a 'export TERM=xterm-256color'

RUN echo '[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh' >> ~/.zshrc && \
    echo 'HISTFILE=/home/node/zsh/.zsh_history' >> ~/.zshrc 

CMD [ \"tail\", \"-f\" , \"/dev/null\" ]
    ".as_bytes()).unwrap();
    Ok(())
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

fn create_p10k_file(project_name: &str) -> std::io::Result<()> {
    let mut p10k = fs::File::create(format!(
        "{}/.docker/zsh/powerlevel10k/.p10k.zsh",
        project_name
    ))?;
    p10k.write_all("".as_bytes()).unwrap();
    Ok(())
}
