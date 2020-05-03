# ProjetRust4IABDS2Lama

## 1 Rappels de Rust, généralités

1.

```
En Rust, les références permettent d'emprunter une variable sans en 
prendre la propriété car En Rust, les variables sont immuables par 
défaut. Tout comme les variables, les références aussi peuvent être 
mutables (que sur une variable mutable et une seule à la fois). "&" 
signifie référence constante et "&mut" signifie référence mutable 
(notons qu'on ne peut avoir les 2 types sur une même variable). Elle
 ne peuvent pas "vivre" plus longtemps que la variable qu'elle référence.
```

2.

```
* les grandes façons de déclarer ses propres types en Rust sont: 
    * struct. _Exemple_ : struct Point { x: int, y: uint }
    * enum. _Exemple_ : enum Forme {
                            Cercle(Point, f64),
                            Rectangle(Point, Point)
    * Tuple. _Exemple_: let tuple3 = ();
```

3.

```
Rust est compilé nativement (assembleur sous forme de code machine).  
```

4.

```
La valeur maximale adressable pour un système avec un processeur de 8bits 
est 2^8 - 1 soit 255 en décimal et FF en Hexadécimal

```

5.

```
Un processus est l’instance d’un programme informatique en cours d’exécution 
par un ou plusieurs threads d’un ordinateur. Concrètement, cela signifie qu’un 
processus permet l’exécution de diverses instructions par le microprocesseur, 
en fonction du programme en cours de fonctionnement.

Source : https://www.journaldunet.fr/web-tech/dictionnaire-du-webmastering/
1445274-process-informatique-definition-detaillee-et-concrete/
```

## 2. Pratique - Micro-shell

### 2.1 Deployement du projet et entrées sorties

#### 2.1.1 Projet binaire avec cargo 

```
Pour Compiler un programme on utilise la commande Cargo build et une fois l'
exécutable généré dans micro-shell/target/debug, on peut l'exécuter avec :
./target/debug/nom_projet.

Pour compiler et exécuter à la fois un programme on peut utiliser la commande 
cargo run sous powershell ou utiliser la commande Run Code lorsque nous
somme sous Visual Code avec tous les extensions installées.

Pour exécuter les tests, nous pouvons effectuer un cargo test (à partir du 
répertoire micro-shell/src/main.rs ) immédiatement pour vérifier si tout 
fonctionne.

Les binaires en mode debug sont rangés dans le repertoire : 
micro-shell/target/rls/incremental

```
#### 2.1.2. Caractère invitant à taper une commande





