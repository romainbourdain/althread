---
sidebar_position: 5
---

# Créer des tests

Nous allons maintenant voir comment créer des tests pour vos processus. Ces tests servent à contrôler les comportements de vos processus et à vérifier qu'ils fonctionnent correctement.

## Blocks de test

En Althread, il existe 3 types de blocks de tests :
- `always`: vérifie qu'une condition est remplie à chaque itération
- `never`: vérifie qu'une condition n'est jamais remplie lors de l'exécution
- `eventually`: vérifie qu'une condition est remplie à un moment donné

Voici un exemple de l'utilisation de ces conditions :
```
shared {
    let x: int;
}

process A() {
    x++;
}

process B() {
    x--;
}

main {
    atomic {
        run A();
        run B();
    }
}

always {
    x < 1;
}
```

:::note
Ici, le block `always` vérifie que la variable `x` est toujours inférieure à 1. Le test ne passera que si le process `B` est exécuté avant le process `A`.
:::

:::info
Il n'est pas possible d'utiliser le block de test pour des variables locales à un processus.
:::

## Fonction assert

La fonction assert permet de vérifier si une condition est remplie. Si la condition n'est pas remplie, le test échoue et affiche un message d'erreur.

Voici un exemple de l'utilisation de la fonction assert :
```
shared {
    let x: int;
}

process A() {
    x++;
}

process B() {
    x--;
}

main {
    atomic {
        run A();
        run B();
    }
    
    assert(x < 1, "x doit être inférieur à 1");
}
```

:::info
Il est possible d'utiliser `assert`sur des variables locales à un processus.
:::