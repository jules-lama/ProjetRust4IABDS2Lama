fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    use std::io::{self, Write};

    let stdout = io::stdout();

    let mut sorti = stdout;
    sorti.write(b">")?;
    // `?` sert à « propager l'erreur » dans la fonction appellante
    // c'est mieux que de crash avec un unwrap ou expect ;)
    sorti.flush()?;

    let mut user_input = String::with_capacity(256);
    stdin.read_line(&mut user_input)?;
    Ok(())
    
}
