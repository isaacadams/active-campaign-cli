#[derive(Default)]
struct Config {}

pub fn load_env_var(name: &'static str) -> String {
    dotenvy::var(name).unwrap_or_else(|_| panic!("missing required env variable: {}", name))
}
