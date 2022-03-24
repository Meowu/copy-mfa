use google_authenticator::GoogleAuthenticator;
use std::env;
use std::process;
use std::error::Error;

const ENV_KEY: &str = "MFA_SECRET";


fn main() -> Result<(), Box<dyn Error>> {

    let mut secret = String::new();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        secret = args[1].clone();
    } else {
        match env::var(ENV_KEY) {
            Ok(token) => {
                secret = String::from(token);
            },
            Err(_) => {
                eprintln!("Missing env variable: {}", ENV_KEY);
                process::exit(1);
            }
        }
    }
    println!("generateing GA code for secrect: {}", secret);
    let auth = GoogleAuthenticator::new();
    let code = auth.get_code(&secret, 0)?;
    println!("generated GA code for given secret {}: {}.", secret, code);
    process::exit(0);
}
