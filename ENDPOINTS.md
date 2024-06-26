# Endpoints Description

### [GET] /summary

#### Description
Returns summary view data for dependent entities ('children') associated with an entity, identified by its *idta*. If no *idta* is specified, the system will return data for the highest-level entities ('parent' entities). The answer will depend on the type of entity.
#### Query parameters

| Key | Required | Type | Ejemplo | Descripción |
|--------|-----------|------|---------|------------ |
| idta | No | number | 2103010100000 | Tecnoandina id entity |

#### Successful response (Código 200)
#### Zones
```json
[
  {
    "idta": 2103010000000,
    "entity_name": "ZONA I",
    "entity_type": "zone",
    "total_sector": 3,
    "region": "Region Norte",
    "total_animals": 84
  },
  {...}
]
```
#### Sectors
```json
[
  {
    "idta": 2103010100000,
    "entity_name": "Purén",
    "entity_type": "sector",
    "total_pavilion": 5,
    "current_breeding": 53,
    "sex": "Hembra",
    "age": "31 dias",
    "total_animals": 457,
    "productive_state": "Abierto",
    "cumulative_mortality": 65,
    "average_profit": 29,
    "conversion": 75,
    "average_weight": 100.0,
    "food": "normal",
    "temperature": "normal",
    "mortality": "danger",
    "connectivity": "warning"
  },
  {...}
]
```
#### Pavilions
```json
[
  {
    "idta": 2103010070010,
    "entity_name": "PABELLON 1",
    "entity_type": "pavilion",
    "total_barnyard": 2,
    "current_breeding": 72,
    "sex": "Hembra",
    "age": "16 dias",
    "total_animals": 88,
    "productive_state": "Abierto",
    "cumulative_mortality": 59,
    "average_profit": 73,
    "conversion": 69,
    "average_weight": 100.0,
    "food": "warning",
    "temperature": "normal",
    "mortality": "danger",
    "connectivity": "normal"
  },
  {...}
]
```
#### Barnyards
```json
[
 {
    "idta": 2103020020021,
    "entity_name": "CORRAL A",
    "entity_type": "farmyard",
    "current_breeding": 97,
    "sex": "Macho",
    "age": "34 dias",
    "total_animals": 84,
    "productive_state": "Abierto",
    "cumulative_mortality": 61,
    "average_profit": 38,
    "conversion": 38,
    "average_weight": 100.0
  },
  {...}
]
```
