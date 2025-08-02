# performance-points-service

Simple microservice to use [MaxOhn](https://github.com/MaxOhn)'s [rosu-pp](https://github.com/MaxOhn/rosu-pp) for calculating of osu! performance points with a simple API.

## Endpoints

### `GET /pp/map`

Returns performance-points values for 100%, 98% and 95%.

Body:
```json
{
    "map_id": 0,
    "mods": 0
}
```
Response:
```json
{
    "pp100": 346.3207669767302,
    "pp98": 274.58196869803527,
    "pp95": 218.67154110108774
}
```

### `GET /pp/score`

Returns the pp and maximum pp for a score.

Body:
```json
{
    "map_id": 2823535,
    "n300": 1493,
    "n100": 164,
    "n50": 6,
    "miss": 3,
    "max_combo": 1455,
    "mods": 0
}
```
Response:
```json
{
    "pp": 164.4639280801979,
    "pp_max": 198.52961826949598
}
```