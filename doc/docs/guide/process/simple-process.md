---
sidebar_position: 1
---


# Utilisation des processus

Nous allons maintenant voir comment créer et exécuter des processus en Althread. Un processus est une unité d'exécution indépendante qui peut s'exécuter en parallèle d'autres processus. Les processus peuvent communiquer entre eux en utilisant des variables partagées ou des canaux.

## Expression atomique

Une expression atomique est la plus petite unité d'exécution. En althread, il existe 5 types d'expressions atomiques :
- **Déclaration** : `let x = 5;`
- **Affectation** : `x = 5;`,  `x++;` ou `x += 1`;
- **Appel de fonction** : `print("Hello world");`
- 
