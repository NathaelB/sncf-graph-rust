# Welcome SNCF-Go

A rest-api to retrieve data related to SNCF traffic.

Construction of graphs to establish different paths available, addition of a layer to superimpose several non-cyclic graphs on top.

The primary objective is to find a route for people with reduced mobility, based on a database of routes submitted by various users. The latter will then be able to obtain a route with a normally high coverage rate.


## Installation

1. Clone repository
```bash
$ git clone git@github.com:NathaelB/sncf-graph-rust.git
$ cd sncf-graph-rust
```

2. Start services

```bash
$ docker compose up -d
```

3. Use ansible for config cluster mongodb
```bash
$ ansible-playbook ./ansible/mongo_cluster.yml
```
3. Run app
```bash
$ cargo run
```


## API Documentation
| Endpoint                   | Méthode | Description                                                       | Paramètres de Requête | Réponse                      |
|----------------------------|---------|-------------------------------------------------------------------|-----------------------|------------------------------| 
| `/routes`                  | GET     | Récupère l'ensemble des routes                                    | - page<br>- size<br>  | `200 OK`                     |
| `/routes/{route_id}/trips` | GET     | Récupère l'ensemble des trajets contenants ses arrêts d'une route | - page<br>- size<br>  | `200 OK`<br> `404 Not Found` |
| `/routes/trips`            | GET     | Récupère l'ensemble des routes avec son nombre de trajets         | - page<br>- size<br>  | `200 OK`<br> `404 Not Found` |

## License

This project is licensed under the [MIT license](https://github.com/nathaelb/sncf-graph-rust/blob/master/LICENSE)