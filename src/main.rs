use google_authenticator::GoogleAuthenticator;
use std::env;
use std::process;
use std::error::Error;
use arboard::Clipboard;

const ENV_KEY: &str = "MFA_SECRET";


fn main() -> Result<(), Box<dyn Error>> {

    let mut clipboard = Clipboard::new().unwrap();

    let secret;

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

    let auth = GoogleAuthenticator::new();
    let code = auth.get_code(&secret, 0)?;

    clipboard.set_text(code).unwrap();
	println!("GA number copied: {}", clipboard.get_text().unwrap());

    process::exit(0);
}
