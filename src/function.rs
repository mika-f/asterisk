use dirs;
use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::ToString;
use toml;

use crate::error::{Error, Result};
use crate::hooks::Hooks;
use crate::shells::{Shell, ShellExecutable};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Function {
    name: String,

    command: String,

    description: Option<String>,

    wrap: Option<String>,

    condition: Option<String>,

    shell: Option<String>,

    hooks: Option<Hooks>,

    tags: Vec<String>,
}

impl Function {
    pub fn new(name: &str, command: &str) -> Self {
        Function {
            name: name.to_owned(),
            command: command.to_owned(),
            description: None,
            wrap: None,
            condition: None,
            shell: None,
            hooks: None,
            tags: vec![],
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_command(&self) -> String {
        self.command.clone()
    }

    pub fn set_description(&mut self, description: Option<String>) -> () {
        self.description = description;
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn set_wrap(&mut self, wrap: Option<String>) -> () {
        self.wrap = wrap;
    }

    pub fn get_wrap(&self) -> Option<String> {
        self.wrap.clone()
    }

    pub fn set_condition(&mut self, condition: Option<String>) -> () {
        self.condition = condition;
    }

    pub fn get_condition(&self) -> Option<String> {
        self.condition.clone()
    }

    pub fn set_shell(&mut self, shell: Option<Shell>) -> () {
        self.shell = match shell {
            Some(shell) => Some(shell.to_string()),
            None => None,
        };
    }

    pub fn get_shell(&self) -> Result<Shell> {
        let shell = self.shell.as_ref();
        match Shell::from_str(&shell.unwrap_or(&"default".to_owned())) {
            Ok(shell) => Ok(shell),
            Err(_) => Err(Error::ShellNotFound(
                shell.unwrap_or(&"default".to_owned()).to_string(),
            )),
        }
    }

    pub fn set_hooks(&mut self, hooks: Option<Hooks>) -> () {
        self.hooks = hooks;
    }

    pub fn get_hooks(&self) -> Option<Hooks> {
        self.hooks.clone()
    }

    pub fn set_tags(&mut self, tags: Vec<String>) -> () {
        self.tags = tags;
    }

    pub fn append_tag(&mut self, tag: String) -> () {
        self.tags.push(tag);
    }

    pub fn remove_tag(&mut self, tag: String) -> () {
        self.tags.retain(|f| f.cmp(&tag) != Ordering::Equal);
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub async fn execute(&self, args: Vec<String>) -> Result<()> {
        let command = self.get_command().clone();
        let shell: Box<dyn ShellExecutable> = Box::new(self.get_shell().unwrap_or(Shell::Default));

        if let Some(hooks) = self.get_hooks() {
            let r = match hooks.get_pre() {
                Some(hook) => shell.exec(&hook).is_ok(),
                None => true,
            };

            if !r {
                return Err(Error::CommandPreHookFailureError(command));
            }
        }

        if let Some(condition) = self.get_condition() {
            let r = shell.exec(&condition).is_ok();

            if !r {
                return Err(Error::CommandConditionFailureError(command));
            }
        }

        let mut run_command = vec![self.get_command().to_owned()];
        run_command.extend(args);

        let s = shell.exec(&run_command.join(" "));
        if !s.is_ok() {
            return Err(Error::CommandExecutionFailureError(command));
        }

        if let Some(hooks) = self.get_hooks() {
            let r = match hooks.get_post() {
                Some(hook) => shell.exec(&hook).is_ok(),
                None => true,
            };

            if !r {
                return Err(Error::CommandPostHookFailureError(command));
            }
        }

        Ok(())
    }

    pub fn export(&self, shell: Shell) -> Option<String> {
        if self.get_wrap().is_none() {
            let command = format!("ast exec {}", self.get_name());
            shell.alias(&self.get_name(), &command)
        } else {
            let wrap = self.get_wrap().unwrap();
            let command = format!("ast exec {} -- ", &wrap);
            shell.alias(&wrap, &command)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Functions {
    functions: Vec<Function>,

    #[serde(skip)]
    cur: usize,
}

impl Functions {
    pub fn load() -> Result<Self> {
        let path = Functions::path()?;
        if path.exists() {
            let toml_str = match fs::read_to_string(&path) {
                Ok(string) => string,
                Err(e) => return Err(Error::OsError(format!("could not read file: {}", e))),
            };

            match toml::from_str::<Functions>(&toml_str) {
                Ok(value) => Ok(value),
                Err(e) => Err(Error::OsError(format!("could not parse toml: {}", e))),
            }
        } else {
            Ok(Functions::none())
        }
    }

    fn none() -> Self {
        Functions {
            functions: vec![],
            cur: 0,
        }
    }

    pub fn path() -> Result<PathBuf> {
        let home = match dirs::config_dir() {
            Some(home) => home,
            None => return Err(Error::OsError("could not find config directory".to_owned())),
        };

        Ok(home.join(".asterisk").join("functions.toml"))
    }

    pub fn append(&mut self, function: Function) -> () {
        self.functions.push(function);
    }

    pub fn delete(&mut self, function: Function) -> () {
        self.functions
            .retain(|f| f.get_name() != function.get_name());
    }

    pub fn get(&self, name: &str) -> Option<&Function> {
        self.functions
            .iter()
            .find(|f| f.get_name() == name || f.get_wrap() == None)
    }

    pub fn get_wrap(&self, name: &str, wrap: &str) -> Option<&Function> {
        self.functions
            .iter()
            .find(|f| f.get_name() == name && f.get_wrap() == Some(wrap.to_owned()))
    }

    pub fn save(&self) -> Result<()> {
        let path = Functions::path()?;
        let root = path.parent().unwrap();

        match fs::create_dir_all(&root) {
            Ok(_) => {}
            Err(e) => return Err(Error::OsError(format!("could not create directory: {}", e))),
        };

        let toml_str = match toml::to_string_pretty(self) {
            Ok(string) => string,
            Err(e) => return Err(Error::OsError(format!("could not serialize toml: {}", e))),
        };

        match fs::write(&path, toml_str) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::OsError(format!("could not write file: {}", e))),
        }
    }
}

impl Iterator for Functions {
    type Item = Function;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let item = self.functions.get(self.cur);
        self.cur += 1;

        match item {
            Some(item) => Some(item.clone()),
            None => None,
        }
    }
}
