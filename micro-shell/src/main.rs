use std::io::{self, Write};
use std::process::{Command, Stdio};

fn redirection() {
    let commande1 = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let commande1_out = Stdio::from(commande1.stdout.expect("failed to execute process"));

    let commande2 = Command::new("echo")
        .arg("Hello, world!")
        .stdin(commande1_out)
        .spawn()
        .expect("failed to execute process");
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut sorti = stdout;

    sorti.write(b">")?;
    // `?` sert à « propager l'erreur » dans la fonction appellante
    // c'est mieux que de crash avec un unwrap ou expect ;)
    sorti.flush()?;

    let mut entrebash = String::with_capacity(300);
    stdin.read_line(&mut entrebash)?;

    let status = Command::new(entrebash.trim())
        .status()
        .expect("failed to execute process");

    println!("process exited with: {}", status);
    assert!(status.success());

    redirection();

    Ok(())
    //Source : https://doc.rust-lang.org/1.34.0/std/io/struct.Stdout.html
}
