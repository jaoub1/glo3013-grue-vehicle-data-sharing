# DIII Projet Multidisciplinaire 2024 -- Networking

Réseautique dans notre ville portuaire intelligente.

Ce répertoire présente la proposition de l'équipe 7 et 10 pour implémenter l'inter-connectivité entre les grues et les véhicules avec un serveur centralisé qui peut être runné à partir de la ligne de commande, d'un Docker container ou accéder directement par une adresse Web. L'[API](#api) HTTP exposée par le serveur permet aux grues de publier ses informations les plus récentes sur le serveur pour le mettre à jour, et aux véhicules de recueillir toutes les informations courrantes pour le stockage des marchandises présentement sur le marché.



## Usage
### Utilisation avec la version déployer par nos soins (Website)
Le lien de l'API hosté sur le Web et disponible 24/7 est le suivant :

http://production.eba-fkhzhyn3.ca-central-1.elasticbeanstalk.com


### Execution dans un Docker
```sh
$ docker build --tag 'grue-vehicle-sharing' .
$ docker run -it 'grue-vehicle-sharing'
```

>Note: You must have [Docker installed](https://docs.docker.com/engine/install/) first.


### Execution via le CLI
L'api est fait en rust, il vous faudra donc installer rust pour l'executer localement : https://www.rust-lang.org/tools/install. Rust se chargera d'installer les dépendances lui même.
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
### POST /grue
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
        String: (team number invalid, invalid JSON)
    - **422** UNPROCESSABLE ENTITY\
        String: (invalid type, missing field)

### GET /vehicle
Fetch toutes les données courrantes des marchandises délivrées par les grues.

- Response:
    -  **200** OK:
    ```json
    {
        "vehicle_data": {
            "team1": 42,
            "team2": 29,
            "team3": 18,
            "team4": 42,
            "team5": 0,
            "team6": 1
        }
    }
    ```

### POST /reset
Fait le reset de toutes les données contenues sur le serveur. Cette route peut être dans deux états différents : protégée (pour éviter les resets volontaires des compétiteurs pour brouiller les données), ou non protégée.

- Body:
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

> Note: Tous les chiffres sont des "JSON numbers" (unsigned integer of 8 bits)

### GET /health
Cette route permet à AWS d'effectuer du monoring sur la ressource. Il est donc très important de garder cette route active et fonctionnel. 

- Response:
    -  **200** OK

### GET /version
Permet d'obtenir la version du package qui run présentement. 

- Response:
    -  **200** OK


## Utilisation de l'API en Python
Pour appeler l'[API](#api) du serveur, il faut réaliser un call HTTP avec la méthode POST ou GET sur les chemins mentionnés dans l'[API](#api).

> Note: Toues les exemples utilisent l'adresse `127.0.0.1:8081` pour travailler localement; elle peut être remplacée par http://production.eba-fkhzhyn3.ca-central-1.elasticbeanstalk.com.

### Grue
```python
body = { "grue_id": 4, "number_of_merchandise": 42 }
response = requests.post("http://127.0.0.1:8081/grue", json = body)

if response.status_code is 200:
    pass # good !
else
    pass # handle error
```

### Véhicule
```python
response = requests.get("http://127.0.0.1:8081/vehicle")

if response.status_code is 200:
    print(response.text) # good !
else
    pass # handle error
```



> *(made in [**Rust**](https://www.rust-lang.org/)* 🦀 *, btw !)*

