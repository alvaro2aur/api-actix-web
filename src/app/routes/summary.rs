use actix_web::{get, web, HttpResponse, Responder};
use rand::seq::SliceRandom;
use rand::Rng;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, SelectTwo};

use crate::app::models::prelude::*;
use crate::app::models::*;

use super::models::{CommomBody, Info, LevelFour, LevelOne, LevelThree, LevelTwo, ResponseData};

#[get("")]
pub async fn get_summary(query: web::Query<Info>, body: web::Json<CommomBody>) -> impl Responder {
    let idta = query.idta;
    let client_db_schema = &body.client.client_db_schema;

    let db = match crate::app::db::init_pool(client_db_schema.clone()).await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to connect to database"),
    };

    let query_result: SelectTwo<Entities, EntityTypes> = if let Some(idta) = idta {
        Entities::find()
            .find_also_related(EntityTypes)
            .filter(entities::Column::ParentId.eq(idta))
    } else {
        Entities::find()
            .find_also_related(EntityTypes)
            .filter(entities::Column::ParentId.is_null())
    };

    let data_result: Result<Vec<(entities::Model, Option<entity_types::Model>)>, sea_orm::DbErr> =
        query_result.all(&db).await;

    let data: Vec<(entities::Model, Option<entity_types::Model>)> = match data_result {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().body("Error fetching data"),
    };

    let response: Vec<ResponseData> = data
        .iter()
        .map(|(entity, entity_type)| create_response(entity, entity_type))
        .collect();

    HttpResponse::Ok().json(response)
}

fn create_response(
    entity: &entities::Model,
    entity_type: &Option<entity_types::Model>,
) -> ResponseData {
    let mut rng = rand::thread_rng();

    let entity_name = entity.name.to_string();
    let entity_type_name = match &entity_type {
        Some(model) => model.name.to_string(),
        None => "No disponible".to_string(),
    };

    match entity.id_entity_type {
        1 => ResponseData::CommomLevelOne(LevelOne {
            idta: entity.idta,
            entity_name: entity_name,
            entity_type: entity_type_name,
            total_sector: 3,
            region: ["Region Norte", "Region Sur"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
            total_animals: rng.gen_range(50..127),
        }),
        2 => ResponseData::CommomLevelTwo(LevelTwo {
            idta: entity.idta,
            entity_name: entity_name,
            entity_type: entity_type_name,
            total_pavilion: 5,
            current_breeding: rng.gen_range(50..127),
            sex: ["Hembra", "Macho"].choose(&mut rng).unwrap().to_string(),
            age: format!("{} dias", rng.gen_range(1..40)),
            total_animals: rng.gen_range(50..500),
            productive_state: "Abierto".to_string(),
            cumulative_mortality: rng.gen_range(1..100),
            average_profit: rng.gen_range(1..100),
            conversion: rng.gen_range(1..100),
            average_weight: 100.0,
            food: ["normal", "warning", "danger"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
            temperature: ["normal", "warning", "danger"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
            mortality: ["normal", "warning", "danger"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
            connectivity: ["normal", "warning", "danger"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
        }),
        3 => ResponseData::CommomLevelThree(LevelThree {
            idta: entity.idta,
            entity_name: entity_name,
            entity_type: entity_type_name,
            current_breeding: rng.gen_range(50..127),
            total_barnyard: rng.gen_range(2..4),
            sex: ["Hembra", "Macho"].choose(&mut rng).unwrap().to_string(),
            age: format!("{} dias", rng.gen_range(1..40)),
            total_animals: rng.gen_range(50..100),
            productive_state: "Abierto".to_string(),
            cumulative_mortality: rng.gen_range(1..100),
            average_profit: rng.gen_range(1..100),
            conversion: rng.gen_range(1..100),
            average_weight: 100.0,
            food: ["normal", "warning", "danger"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
            temperature: ["normal", "warning", "danger"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
            mortality: ["normal", "warning", "danger"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
            connectivity: ["normal", "warning", "danger"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
        }),
        4 => ResponseData::CommomLevelFour(LevelFour {
            idta: entity.idta,
            entity_name: entity_name,
            entity_type: entity_type_name,
            current_breeding: rng.gen_range(50..127),
            sex: ["Hembra", "Macho"].choose(&mut rng).unwrap().to_string(),
            age: format!("{} dias", rng.gen_range(1..40)),
            total_animals: rng.gen_range(50..100),
            productive_state: "Abierto".to_string(),
            cumulative_mortality: rng.gen_range(1..100),
            average_profit: rng.gen_range(1..100),
            conversion: rng.gen_range(1..100),
            average_weight: 100.0,
        }),
        _ => panic!("Unexpected entity type"),
    }
}
