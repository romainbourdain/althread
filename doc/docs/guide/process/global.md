---
sidebar_position: 3
---


# Variables partagées

Jusqu'à présent, les variables déclarées dans un processus sont locales à ce processus. Cela signifie qu'un processus ne peut pas accéder aux variables des autres processus :

```
process Process1() {
    print(x); // x n'existe pas dans ce processus
}

main {
    let x = 0;
    run Process1();
}
```
:::danger
Le code ci-dessus renverra une erreur : le processus `Process1` ne peut pas accéder à la variable `x` déclarée dans le processus principal.
:::


## Déclaration de variables partagées

Pour permettre à plusieurs processus d'accéder à une même variable, vous devez la déclarer comme une variable partagée. Une variable partagée est une variable qui peut être lue et modifiée par plusieurs processus. Voici comment déclarer une variable partagée :

```
shared {
    let x: int;
    let y = false;
    const a = 42;
}
```

:::tip
Les déclaration du block `shared` fonctionnent comme les déclarations classiques : elles peuvent être constantes ou mutables, avoir n'importe quel type et l'on peut leur assigner une valeur
:::

:::warning
Il n'est possible de faire que des déclarations dans le block `shared`.
:::

## Exécution de processus avec des variables partagées

Lors de l'exécution, le block `shared` est exécuté d'une traite avant les processus. Les variables partagées sont ainsi accessibles et modifiables par tous les processus.

```
shared {
    let x : int;
}

process Process1() {
    x++;
    wait(x == 2);
}

main {
    run Process1();
    run Process1();
}
```

:::note
Dans cet exemple, les deux processus `Process1` incrémentent la variable `x` de 1. Le premier processus attend ensuite que `x` soit égal à 2 avant de continuer.
:::