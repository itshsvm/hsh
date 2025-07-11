pub mod Variables {
    use std::{collections::HashMap, env, sync::Mutex};

    use once_cell::sync::Lazy;

    pub static ENV_VARS: Lazy<Mutex<HashMap<String, String>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));

    pub fn set_var(key: &str, value: &str) {
        let mut env = ENV_VARS.lock().unwrap();
        env.insert(key.to_string(), value.to_string());
    }

    pub fn export_var(key: &str, value: &str) {
        let mut env = ENV_VARS.lock().unwrap();
        env.insert(key.to_string(), value.to_string());
        unsafe { env::set_var(key, value) };
    }

    pub fn get_var(key: &str) -> Option<String> {
        let env = ENV_VARS.lock().unwrap();
        env.get(key).cloned()
    }

    pub fn print_all_vars() {
        let env = ENV_VARS.lock().unwrap();

        for (key, value) in env.iter() {
            println!("{} = {}", key, value);
        }
    }

    pub fn get_all_vars() -> HashMap<String, String> {
        let env = ENV_VARS.lock().unwrap();
        env.clone()
    }

    pub fn remove_var(key: &str) {
        let mut vars = ENV_VARS.lock().unwrap();
        vars.remove(key);
    }
}
