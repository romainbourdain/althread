---
sidebar_position: 3
---

# Syntaxe d'Althread

La syntaxe d'althread est faite pour être la plus intuitive possible. Elle est inspirée du langage C, ce qui permet de la prendre en main rapidement et de se concentrer sur les concepts plutôt que sur la syntaxe.


## Commentaires

Il existe deux types de commentaires en althread :
- **Commentaire sur une ligne** : `// Ceci est un commentaire sur une ligne`
- **Commentaire sur plusieurs lignes** : `/* Ceci est un commentaire sur plusieurs lignes */`

## Structure d'un programme

Un programme est structuré en plusieurs blocks, qui peuvent correspondre à 3 types d'éléments :
- **Déclaration de variables globales** : `shared { ... }`
- **Vérification de conditions** : `always { ... }` ou `never { ... }`
- **Définition de processus** : `process A() { ... }` ou `main { ... }`

Le block main est le processus principal. Il est exécuté en premier et sert de point d'entrée au programme.

## Type de données

Les variables en althread peuvent prendre les types suivants :
- **Vide** : `void`
- **Booléen** : `bool`
- **Entier** : `int`
- **Flottant** : `float`
- **Chaîne de caractères** : `string`


### Typage statique

Althread utilise un typage statique ce qui signifie que le type d'une variable est déterminé lorsqu'elle est déclarée et ne peut pas être modifié par la suite. Ainsi, le programme suivant provoquera une erreur :

```
let x: int = 5;
x = 3.4; // Erreur : x est de type int et ne peut pas prendre de valeur de type float.
```

### Typage implicite

```
let a: int = 5;   // x est de type int et prend la valeur 5.
let b: bool;      // x est de type bool et prend la valeur par défaut false.
let c = 3.4;      // x est de type float et prend la valeur 3.4.
let d;            // x est de type void et prend la valeur par défaut `null`.
```

## Expression atomique

Une expression atomique est la plus petite unité d'exécution. En althread, il existe 6 types d'expressions atomiques :
- **Déclaration** : `let x = 5;`
- **Affectation** : `x = 5;`,  `x++;`, `x += 1`;
- **Opération arithmétique** : `x + y;`, `x - y;`, `x * y;`, `x / y;`, `x % y;`
- **Scope atomique**: `atomic { ... }`
- **Appel de fonction** : `print("Hello world");`, `wait(x == 5);`
- **Exécution de processus** : `run A();`

Les expressions atomiques ne peuvent pas être interrompues par un autre processus. Cela signifie que pendant qu'un processus exécute une expression atomique, aucun autre processus ne peut prendre la main.

## Structures de contrôle et portée des variables

Althread propose plusieurs structures de contrôle pour gérer le flux d'exécution d'un programme :
- **Condition** : `if (condition) { ... } else { ... }`
- **Boucle** : `while (condition) { ... }`
- **Scope** : `{ ... }`

Les variables déclarées dans une structure de contrôle sont visibles uniquement à l'intérieur de cette structure. Cela permet de limiter la portée des variables et d'éviter les conflits de noms. 

Il est possible au sein d'une structure de contrôle de déclarer une variable avec le même nom qu'une variable globale. Dans ce cas, la variable locale masque la variable globale.