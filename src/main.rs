use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    if let Err(e) = (|| -> io::Result<()> {
        logo()?;
        wait_for_enter()?;
        Ok(())
    })() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn logo() -> io::Result<()> {
    let logo_content = fs::read_to_string("assets/logo.txt")?;

    println!();
    for line in logo_content.lines() {
        println!("{}", line);
        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}

fn wait_for_enter() -> io::Result<()> {
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
