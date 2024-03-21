# DIII Projet Multidisciplinaire 2024 -- Networking

Réseautique dans notre ville portuaire intelligente.

Ce répertoire présente la proposition de l'équipe 7 et 10 pour implémenter l'inter-connectivité entre les grues et les véhicules avec un serveur centralisé qui peut être runné à partir de la ligne de commande, d'un Docker container ou accéder directement par une adresse Web. L'[API](#api) HTTP exposée par le serveur permet aux grues de publier ses informations les plus récentes sur le serveur pour le mettre à jour, et aux véhicules de recueillir toutes les informations courrantes pour le stockage des marchandises présentement sur le marché.



## Content
[Installation](#installation)\
[Usage](#usage)\
[API](#api)\
[Exemple d'utilisation de l'API en Python](#exemple-dutilisation-de-lapi-en-python)



## Installation
Pour utiliser l'API du serveur HTTP de la manière présentée dans les [exemples](#exemple-dutilisation-de-lapi-en-python) ci-dessous, vous devez installer les *minimum requirements* suivants :

- [Python +3.8](https://www.python.org/downloads/)
- [Docker +24.0](https://docs.docker.com/engine/install/) (optional) :
    Permet d'utiliser le serveur sans devoir setup l'environnement sur un ordinateur.
- [Rust +1.76](https://www.rust-lang.org/tools/install) (optional) :
    Permet d'installer les toolchains pour compiler et faire plusieurs commandes utiles pour le développement en [Rust](https://www.rust-lang.org/).

> Note: Tous ces programmes peuvent être installés sur GNU+Linux, MacOS et Windows.



## Usage
Il existe différentes manières d'utiliser le serveur HTTP :
- [Version Web sur Amazon Web Services (AWS)](#version-web-sur-amazon-web-services-aws)
- [Exécution dans Docker](#exécution-dans-un-docker)
- [Exécution via le CLI](#exécution-via-cargo)

### Version Web sur Amazon Web Services (AWS)
Le lien de l'API hosté sur le Web et disponible 24/7 est le suivant :
```
http://production.eba-fkhzhyn3.ca-central-1.elasticbeanstalk.com:80/
```

> Note: C'est le port `80` (HTTP) qui est utilisé pour intéragir avec le serveur.


### Exécution dans un Docker
```sh
$ docker build --tag 'grue-vehicle-sharing' -f Dockerfile_rust .
$ docker run --network=host -it 'grue-vehicle-sharing'
```

>Note: You must have [Docker installed](#installation) first.


### Exécution via Cargo
L'API est écrite en [Rust](#installation) ; on peut donc l'exécuter directement avec `cargo` et les dépendances se compileront par elles-mêmes.
```sh
$ cargo run --release -- --address 127.0.0.1 --port 8081

###############################
Usage: axum_prototype [OPTIONS]

Options:
  -a, --address <ADDRESS>      Address of the TCP connection [default: 0.0.0.0]
  -p, --port <PORT>            TCP port number [default: 8080]
  -l, --lock-uuid <LOCK_UUID>  Specific lock UUIDv4
  -h, --help                   Print help
  -V, --version                Print version
```

> Note: You must have [Rust installed](https://www.rust-lang.org/tools/install) first.

### Useful commands
```sh
$ cargo test       # Run tests
$ cargo lint       # Lint all code
$ cargo fmt        # Format all code
$ cargo doc --open # Open offline documentation
```



## API
> Note: Tous les chiffres sont des "JSON numbers" (unsigned integer of 8 bits)

### POST `/grue`
Permet de d'envoyer le nombre de marchandises que possède votre grue.
- Body:
    ```json
    {
        "grue_id": 4,
        "number_of_merchandise": 42
    }
    ```
    
- Response:
    - **200** OK
    - **400** BAD_REQUEST\
        String: (grue_id invalid, invalid JSON)
    - **422** UNPROCESSABLE ENTITY\
        String: (invalid type, missing field)

### GET `/vehicle`
Fetch toutes les données courrantes des marchandises délivrées par les grues.

- Response:
    -  **200** OK:
    ```json
    {
        "vehicle_data": {
            "zone1": 42,
            "zone2": 29,
            "zone3": 18,
            "zone4": 42,
            "zone5": 0,
            "zone6": 1
        }
    }
    ```

> Note: La range valide pour les zones de changements est entre `0` et `99`. Si une zone n'est pas listée, alors aucune marchandise n'a été reportée pour celle-ci.

> Note: Par défaut, les zones 1 à 6 sont toujours setter à 0, et les autres sont absentes.

### POST `/reset`
Fait le reset de toutes les données contenues sur le serveur. Cette route peut être dans deux états différents : protégée (pour éviter les resets volontaires des compétiteurs pour brouiller les données), ou non protégée.

- Body (optional):
    ```json
    {
        "uuid": "ffffffff-ffff-ffff-ffff-ffffffffffff"
    }
    ```
- Response:
    -  **200** OK
    -  **400** BAD_REQUEST\
        String (invalid uuid, reset not allowed)
    - **422** UNPROCESSABLE ENTITY\
        String: (invalid type, missing field)

### GET `/health`
Cette route permet à AWS d'effectuer du monoring sur la ressource. Il est donc très important de garder cette route active et fonctionnelle.

- Response:
    -  **200** OK

### GET `/version`
Permet d'obtenir la version du package qui run présentement. 

- Response:
    -  **200** OK\
        Version in a string (e.g. "0.5.2")



## Exemple d'utilisation de l'API en Python
Pour appeler l'[API](#api) du serveur, il faut réaliser un call HTTP avec la méthode POST ou GET sur les chemins mentionnés dans l'[API](#api).

> Note: Tous les exemples utilisent l'adresse `127.0.0.1:8081` pour travailler localement, mais elle peut être remplacée par une autre, telle que celle sur AWS [ici](#version-web-sur-amazon-web-services-aws).

### Grue
```python
import requests

body = { "grue_id": 4, "number_of_merchandise": 42 }
response = requests.post("http://127.0.0.1:8081/grue", json = body)

if response.status_code == 200:
    pass # good !
else:
    pass # handle error
```

### Véhicule
```python
import requests

response = requests.get("http://127.0.0.1:8081/vehicle")

if response.status_code == 200:
    print(response.text) # good !
else:
    pass # handle error
```



> *(made in [**Rust**](https://www.rust-lang.org/)* 🦀 *, btw !)*

