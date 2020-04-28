# ProjetRust4IABDS2Lama

## 1 Questions : Rappels de Rust, généralités

1.

```
En Rust, les références permettent d'emprunter une variable sans en prendre la propriété. Tout comme les variables, les références aussi peuvent être mutables (que sur une variable mutable et une seule à la fois). "&" signifie référence constante et "&mut" signifie référence mutable (notons qu'on ne peut avoir les 2 types sur une même variable). Elle ne peuvent pas "vivre" plus longtemps que la variable qu'elle référence.
```

2.

```les grandes façons de déclarer ses propres types en Rust sont: 
    * struct. _Exemple_ : struct Point { x: int, y: uint }
    * enum. _Exemple_ : enum Forme {
                            Cercle(Point, f64),
                            Rectangle(Point, Point)
    * Tuple. _Exemple_: let tuple3 = ();
```

3.

