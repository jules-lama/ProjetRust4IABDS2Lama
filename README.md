# ProjetRust4IABDS2Lama

## 1 Questions : Rappels de Rust, généralités

1.

```
En Rust, les références permettent d'emprunter une variable sans en prendre la propriété car En Rust, les variables sont 
immuables par défaut. Tout comme les variables, les références aussi peuvent être mutables (que sur une variable 
mutable et une seule à la fois). "&" signifie référence constante et "&mut" signifie référence mutable (notons qu'on 
ne peut avoir les 2 types sur une même variable). Elle ne peuvent pas "vivre" plus longtemps que la variable qu'elle 
référence.
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
La valeur maximale adressable pour un système avec un processeur de 8bits est 2^8 soit 256 en décimal et 100 en Hexadécimal

```

5.

```
**un processus** est l’instance d’un programme informatique en cours d’exécution par un ou plusieurs threads d’un ordinateur. Concrètement, cela signifie qu’un processus permet l’exécution de diverses instructions par le microprocesseur, en fonction du programme en cours de fonctionnement.

```
```
**Source** : https://www.journaldunet.fr/web-tech/dictionnaire-du-webmastering/1445274-process-informatique-definition-detaillee-et-concrete/
```