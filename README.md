# DIII Projet Multidisciplinaire 2024 -- Networking

Réseautique dans notre ville portuaire intelligente.

Ce répertoire présente la proposition de l'équipe XXX et 10 pour implémenter l'inter-connectivité entre les grues, les véhicules avec un serveur centralisé qui peut être runné à partir de la ligne de commande, d'un Docker container ou accéder directement par le Web à l'adresse `http://{to be defined}/`. L'API exposée par le serveur permet aux grues de publier les informations les plus récentes sur le serveur pour le mettre à jour, et aux véhicules de recuillir toutes les informations courrantes pour le stockage des marchandises présentement sur le marché.

Cette interaction se fait à l'aide de l'API HTTP suivante :

### POST /grue

- Body:
    ```json
    {
        "grue_id": 4,
        "number_of_merchandise": 42
    }
    ```
- Response:
    - 200 OK
    - 400 BAD_REQUEST\
        String: (team number invalid)

### GET /vehicle

- Response:
    -  200 OK:
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

> Note: Tous les chiffres sont des JSON number (unsigned integer of 8 bits)



## Utilisation de l'API
L'utilisation se fait avec un call POST ou GET à l'API. En Python, voici comment faire 

> Note: Toues les exemples utilisent l'adresse `127.0.0.1:8081` ; elle peut être remplacée par la bonne adresse lorsqu'elle sera déterminée.

## Grue
```python
body = { "grue_id": 4, "number_of_merchandise": 42 }
response = requests.post("http://127.0.0.1:8081/grue", json = body)

if response.status_code is 200:
    pass # good !
else
    pass # handle error
```

## Véhicule
```python
response = requests.get("http://127.0.0.1:8081/vehicle")

if response.status_code is 200:
    print(response.text) # good !
else
    pass # handle error
```



> *(made in **[Rust](https://www.rust-lang.org/) 🦀**, btw !)*