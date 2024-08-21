---
sidebar_position: 1
---


# Utilisation des processus

Nous allons maintenant voir comment créer et exécuter des processus en Althread. Un processus est une unité d'exécution indépendante qui peut s'exécuter en parallèle d'autres processus. Les processus peuvent communiquer entre eux en utilisant des variables partagées ou des canaux.

## Déclaration d'un processus

Pour déclarer un processus, vous devez utiliser le mot-clé `process`. Voici un exemple de déclaration de processus :

```
process MyProcess() {
    // code du processus
}
```

:::note
Il est possible de déclarer autant de processus que vous le souhaitez. Tous les processus déclarés sont stockés dans une liste
:::
:::warning
Il n'est pas possible d'avoir deux processus avec le même nom.
:::

## Exécution d'un processus

Pour exécuter un processus, vous devez utiliser la fonction `run`. Voici un exemple d'exécution d'un processus :

```
main {
    run MyProcess();
}
```

:::note
Un processus peut être exécuté plusieurs fois en parallèle :
```
main {
    run MyProcess();
    run MyProcess();
}
```
:::

### Que se passe-t-il lorsqu'un processus est exécuté ?

Un programme althread est exécuté par itération. Chaque itération correspond à l'exécution d'une [instruction atomique](/docs/guide/getting-started/syntaxe#expression-atomique) d'un processus choisi aléatoirement parmi les processus en cours d'exécution. Lorsqu'un processus est exécuté, il peut effectuer des opérations telles que l'assignation de variables, l'appel de fonctions, la lecture ou l'écriture de canaux, etc...

## Exemple complet

Voici un exemple complet d'un programme Althread qui exécute deux processus en parallèle :

```
process Process1() {
    print("process 1");
}

main {
    run Process1();
    print("main");
}
```

Dans cet exemple, le processus `Process1` est exécuté en parallèle du processus principal. Voici comment s'exécute ce programme :
1. Le processus `Process1` et le processus principal sont déclarés et stockés dans la liste des processus.
2. Le processus principal est ajouté à la liste des processus en cours d'exécution.
3. Un processus est tiré aléatoirement parmi les processus en cours d'exécution. Ici, comme il n'y a que le processus principal, c'est lui qui est exécuté.
4. Le processus principal exécute l'instruction `run Process1();`, ce qui ajoute le processus `Process1` à la liste des processus en cours d'exécution.
5. Un processus est tiré aléatoirement parmi les processus en cours d'exécution. Ici, le processus principal et le processus `Process1` sont en cours d'exécution, donc l'un des deux est exécuté aléatoirement (soit l'instruction `print("main");`, soit l'instruction `print("process 1");`).
6. Quand un processus a terminé son exécution, il est retiré de la liste des processus en cours d'exécution.
7. Quand tous les processus ont terminé leur exécution, le programme s'arrête.


:::note
Il n'y a pas de priorité quant à l'ordre de déclaration des processus : tous les processus déclarés sont stockés dans la liste des processus avant l'exécution du processus principal.
:::
