use std::io::{self, Write};
use std::process::{Command, Stdio};

fn redirection() {

    //Creation de la première commande(processus)
    let commande1 = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    /*Création du tube pour contenir la sorti de la première commande qui sera l'entrée de 
    la deuxième commande*/ 

    let commande1_out = Stdio::from(commande1.stdout.expect("failed to execute process"));

    /*Création de la deuxième commande (processus) qui aura en entrée la sortie du tube (sortie de 
    la première commande)*/

    let commande2 = Command::new("echo")
        .arg("Hello, world!")
        .stdin(commande1_out)
        .spawn()
        .expect("failed to execute process");
}

fn processus_en_fond() {

    //Creation de la commande(processus)
    let mut commandefond = Command::new("echo")
        .arg("Message d'essai")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let stdin = commandefond.stdin.as_mut().expect("failed to get stdin");
        stdin.write_all(b"Verification").expect("failed to write to stdin");

    //Création de la commande (processus) qui va activer l'exécution en fond

    let commandesorti = commandefond
        .wait()
        .expect("failed to wait on child");

}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    //Création d'un caractère invitant à la saisie dans le micro-shell ">"
    let mut sorti = stdout;
        sorti.write(b">")?;

    /* `?` sert à « propager l'erreur » dans la fonction appellante
    c'est mieux que de crash avec un unwrap ou expect ;)*/
        sorti.flush()?;

    //Stockage de la donnée (commande) entrée par l'utilisateur
    let mut entrebash = String::with_capacity(300);
        stdin.read_line(&mut entrebash)?;

    let status = Command::new(entrebash.trim())
        .status()
        .expect("failed to execute process");

    println!("process exited with: {}", status);
    assert!(status.success());

    //redirection();
    processus_en_fond();

    Ok(())
    //Source : https://doc.rust-lang.org/1.34.0/std/io/struct.Stdout.html
}
