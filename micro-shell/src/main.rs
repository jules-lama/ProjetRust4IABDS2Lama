fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    use std::io::{self, Write};

    let stdout = io::stdout();

    let mut sorti = stdout;
    sorti.write(b">")?;
    // `?` sert à « propager l'erreur » dans la fonction appellante
    // c'est mieux que de crash avec un unwrap ou expect ;)
    sorti.flush()?;

    let mut entrebash = String::with_capacity(300);
    stdin.read_line(&mut entrebash)?;

    use std::process::Command; //Création d'un process
    let status = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .status()
        .expect("failed to execute process");

    println!("process exited with: {}", status);
    assert!(status.success());
    //Source : https://doc.rust-lang.org/1.34.0/std/io/struct.Stdout.html

    Ok(())
}
