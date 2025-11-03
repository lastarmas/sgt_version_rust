use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{DateTime, Utc};

use crate::models::{Travail, TypeTravail, Application, Environnement, StatutTravail};

#[derive(Debug, Deserialize)]
pub struct CreateTravailRequest {
    pub projet_id: Uuid,
    pub type_travail: TypeTravail,
    pub application: Application,
    pub environnement: Environnement,
    pub description: String,
    pub date_debut: DateTime<Utc>,
    pub date_fin_prevue: DateTime<Utc>,
    pub statut: StatutTravail,
    pub responsable: Uuid,
    pub equipe: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct TravailResponse {
    pub id: Uuid,
    pub projet_id: Uuid,
    pub type_travail: TypeTravail,
    pub application: Application,
    pub environnement: Environnement,
    pub description: String,
    pub date_debut: DateTime<Utc>,
    pub date_fin_prevue: DateTime<Utc>,
    pub statut: StatutTravail,
    pub responsable: Uuid,
    pub equipe: Vec<Uuid>,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/travaux")
            .route("", web::get().to(get_travaux))
            .route("", web::post().to(create_travail))
            .route("/{id}", web::get().to(get_travail))
            .route("/{id}", web::put().to(update_travail))
            .route("/{id}", web::delete().to(delete_travail))
            .route("/projet/{projet_id}", web::get().to(get_travaux_by_projet))
    );
}

pub async fn get_travaux(travaux: web::Data<Mutex<HashMap<Uuid, Travail>>>) -> Result<HttpResponse> {
    let travaux_map = travaux.lock().unwrap();
    let travaux_list: Vec<TravailResponse> = travaux_map.values()
        .map(|t| TravailResponse {
            id: t.id,
            projet_id: t.projet_id,
            type_travail: t.type_travail.clone(),
            application: t.application.clone(),
            environnement: t.environnement.clone(),
            description: t.description.clone(),
            date_debut: t.date_debut,
            date_fin_prevue: t.date_fin_prevue,
            statut: t.statut.clone(),
            responsable: t.responsable,
            equipe: t.equipe.clone(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(travaux_list))
}

pub async fn get_travail(
    web::Path(id): web::Path<Uuid>,
    travaux: web::Data<Mutex<HashMap<Uuid, Travail>>>,
) -> Result<HttpResponse> {
    let travaux_map = travaux.lock().unwrap();
    
    match travaux_map.get(&id) {
        Some(travail) => {
            let response = TravailResponse {
                id: travail.id,
                projet_id: travail.projet_id,
                type_travail: travail.type_travail.clone(),
                application: travail.application.clone(),
                environnement: travail.environnement.clone(),
                description: travail.description.clone(),
                date_debut: travail.date_debut,
                date_fin_prevue: travail.date_fin_prevue,
                statut: travail.statut.clone(),
                responsable: travail.responsable,
                equipe: travail.equipe.clone(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Travail non trouvé"
        })))
    }
}

pub async fn get_travaux_by_projet(
    web::Path(projet_id): web::Path<Uuid>,
    travaux: web::Data<Mutex<HashMap<Uuid, Travail>>>,
) -> Result<HttpResponse> {
    let travaux_map = travaux.lock().unwrap();
    let travaux_list: Vec<TravailResponse> = travaux_map.values()
        .filter(|t| t.projet_id == projet_id)
        .map(|t| TravailResponse {
            id: t.id,
            projet_id: t.projet_id,
            type_travail: t.type_travail.clone(),
            application: t.application.clone(),
            environnement: t.environnement.clone(),
            description: t.description.clone(),
            date_debut: t.date_debut,
            date_fin_prevue: t.date_fin_prevue,
            statut: t.statut.clone(),
            responsable: t.responsable,
            equipe: t.equipe.clone(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(travaux_list))
}

pub async fn create_travail(
    travail_req: web::Json<CreateTravailRequest>,
    travaux: web::Data<Mutex<HashMap<Uuid, Travail>>>,
) -> Result<HttpResponse> {
    let mut travaux_map = travaux.lock().unwrap();
    
    let travail = Travail {
        id: Uuid::new_v4(),
        projet_id: travail_req.projet_id,
        type_travail: travail_req.type_travail.clone(),
        application: travail_req.application.clone(),
        environnement: travail_req.environnement.clone(),
        description: travail_req.description.clone(),
        date_debut: travail_req.date_debut,
        date_fin_prevue: travail_req.date_fin_prevue,
        statut: travail_req.statut.clone(),
        responsable: travail_req.responsable,
        equipe: travail_req.equipe.clone(),
    };

    travaux_map.insert(travail.id, travail.clone());

    let response = TravailResponse {
        id: travail.id,
        projet_id: travail.projet_id,
        type_travail: travail.type_travail,
        application: travail.application,
        environnement: travail.environnement,
        description: travail.description,
        date_debut: travail.date_debut,
        date_fin_prevue: travail.date_fin_prevue,
        statut: travail.statut,
        responsable: travail.responsable,
        equipe: travail.equipe,
    };

    Ok(HttpResponse::Created().json(response))
}

pub async fn update_travail(
    web::Path(id): web::Path<Uuid>,
    travail_req: web::Json<CreateTravailRequest>,
    travaux: web::Data<Mutex<HashMap<Uuid, Travail>>>,
) -> Result<HttpResponse> {
    let mut travaux_map = travaux.lock().unwrap();
    
    if let Some(existing_travail) = travaux_map.get_mut(&id) {
        existing_travail.projet_id = travail_req.projet_id;
        existing_travail.type_travail = travail_req.type_travail.clone();
        existing_travail.application = travail_req.application.clone();
        existing_travail.environnement = travail_req.environnement.clone();
        existing_travail.description = travail_req.description.clone();
        existing_travail.date_debut = travail_req.date_debut;
        existing_travail.date_fin_prevue = travail_req.date_fin_prevue;
        existing_travail.statut = travail_req.statut.clone();
        existing_travail.responsable = travail_req.responsable;
        existing_travail.equipe = travail_req.equipe.clone();

        let response = TravailResponse {
            id: existing_travail.id,
            projet_id: existing_travail.projet_id,
            type_travail: existing_travail.type_travail.clone(),
            application: existing_travail.application.clone(),
            environnement: existing_travail.environnement.clone(),
            description: existing_travail.description.clone(),
            date_debut: existing_travail.date_debut,
            date_fin_prevue: existing_travail.date_fin_prevue,
            statut: existing_travail.statut.clone(),
            responsable: existing_travail.responsable,
            equipe: existing_travail.equipe.clone(),
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Travail non trouvé"
        })))
    }
}

pub async fn delete_travail(
    web::Path(id): web::Path<Uuid>,
    travaux: web::Data<Mutex<HashMap<Uuid, Travail>>>,
) -> Result<HttpResponse> {
    let mut travaux_map = travaux.lock().unwrap();
    
    match travaux_map.remove(&id) {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Travail non trouvé"
        })))
    }
}
