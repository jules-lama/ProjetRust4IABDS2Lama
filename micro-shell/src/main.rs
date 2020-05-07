fn main() -> std::io::Result<()> {
    
    use std::io::{self, Write};
    use std::process::{Command, Stdio};

    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut sorti = stdout;

        sorti.write(b">")?;
        // `?` sert à « propager l'erreur » dans la fonction appellante
        // c'est mieux que de crash avec un unwrap ou expect ;)
        sorti.flush()?;

    let mut entrebash = String::with_capacity(300);
        stdin.read_line(&mut entrebash)?;

    
    let status = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .status()
        .expect("failed to execute process");

    println!("process exited with: {}", status);
    assert!(status.success());


    let commande1 = Command::new("ls")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let commande1_out = commande1.stdout.unwrap() ;

    let commande2 = Command::new("echo")
        .stdin(commande1_out)
        .spawn()
        .expect("failed to execute process");
    
    let commande3 = commande2.wait_with_output().unwrap();
    assert_eq!(String::from_utf8_lossy(&commande3.stdout), "Hello, world!\n");


    Ok(())
}

//Source : https://doc.rust-lang.org/1.34.0/std/io/struct.Stdout.html