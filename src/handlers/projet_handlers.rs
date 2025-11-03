use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{DateTime, Utc};

use crate::models::{Projet, StatutProjet, Priorite};

#[derive(Debug, Deserialize)]
pub struct CreateProjetRequest {
    pub code: String,
    pub nom: String,
    pub description: String,
    pub date_debut: DateTime<Utc>,
    pub date_fin_prevue: DateTime<Utc>,
    pub statut: StatutProjet,
    pub priorite: Priorite,
}

#[derive(Debug, Serialize)]
pub struct ProjetResponse {
    pub id: Uuid,
    pub code: String,
    pub nom: String,
    pub description: String,
    pub date_debut: DateTime<Utc>,
    pub date_fin_prevue: DateTime<Utc>,
    pub statut: StatutProjet,
    pub priorite: Priorite,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/projets")
            .route("", web::get().to(get_projets))
            .route("", web::post().to(create_projet))
            .route("/{id}", web::get().to(get_projet))
            .route("/{id}", web::put().to(update_projet))
            .route("/{id}", web::delete().to(delete_projet))
    );
}

pub async fn get_projets(projets: web::Data<Mutex<HashMap<Uuid, Projet>>>) -> Result<HttpResponse> {
    let projets_map = projets.lock().unwrap();
    let projets_list: Vec<ProjetResponse> = projets_map.values()
        .map(|p| ProjetResponse {
            id: p.id,
            code: p.code.clone(),
            nom: p.nom.clone(),
            description: p.description.clone(),
            date_debut: p.date_debut,
            date_fin_prevue: p.date_fin_prevue,
            statut: p.statut.clone(),
            priorite: p.priorite.clone(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(projets_list))
}

pub async fn get_projet(
    web::Path(id): web::Path<Uuid>,
    projets: web::Data<Mutex<HashMap<Uuid, Projet>>>,
) -> Result<HttpResponse> {
    let projets_map = projets.lock().unwrap();
    
    match projets_map.get(&id) {
        Some(projet) => {
            let response = ProjetResponse {
                id: projet.id,
                code: projet.code.clone(),
                nom: projet.nom.clone(),
                description: projet.description.clone(),
                date_debut: projet.date_debut,
                date_fin_prevue: projet.date_fin_prevue,
                statut: projet.statut.clone(),
                priorite: projet.priorite.clone(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Projet non trouvé"
        })))
    }
}

pub async fn create_projet(
    projet_req: web::Json<CreateProjetRequest>,
    projets: web::Data<Mutex<HashMap<Uuid, Projet>>>,
) -> Result<HttpResponse> {
    let mut projets_map = projets.lock().unwrap();
    
    let projet = Projet {
        id: Uuid::new_v4(),
        code: projet_req.code.clone(),
        nom: projet_req.nom.clone(),
        description: projet_req.description.clone(),
        date_debut: projet_req.date_debut,
        date_fin_prevue: projet_req.date_fin_prevue,
        statut: projet_req.statut.clone(),
        priorite: projet_req.priorite.clone(),
    };

    projets_map.insert(projet.id, projet.clone());

    let response = ProjetResponse {
        id: projet.id,
        code: projet.code,
        nom: projet.nom,
        description: projet.description,
        date_debut: projet.date_debut,
        date_fin_prevue: projet.date_fin_prevue,
        statut: projet.statut,
        priorite: projet.priorite,
    };

    Ok(HttpResponse::Created().json(response))
}

pub async fn update_projet(
    web::Path(id): web::Path<Uuid>,
    projet_req: web::Json<CreateProjetRequest>,
    projets: web::Data<Mutex<HashMap<Uuid, Projet>>>,
) -> Result<HttpResponse> {
    let mut projets_map = projets.lock().unwrap();
    
    if let Some(existing_projet) = projets_map.get_mut(&id) {
        existing_projet.code = projet_req.code.clone();
        existing_projet.nom = projet_req.nom.clone();
        existing_projet.description = projet_req.description.clone();
        existing_projet.date_debut = projet_req.date_debut;
        existing_projet.date_fin_prevue = projet_req.date_fin_prevue;
        existing_projet.statut = projet_req.statut.clone();
        existing_projet.priorite = projet_req.priorite.clone();

        let response = ProjetResponse {
            id: existing_projet.id,
            code: existing_projet.code.clone(),
            nom: existing_projet.nom.clone(),
            description: existing_projet.description.clone(),
            date_debut: existing_projet.date_debut,
            date_fin_prevue: existing_projet.date_fin_prevue,
            statut: existing_projet.statut.clone(),
            priorite: existing_projet.priorite.clone(),
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Projet non trouvé"
        })))
    }
}

pub async fn delete_projet(
    web::Path(id): web::Path<Uuid>,
    projets: web::Data<Mutex<HashMap<Uuid, Projet>>>,
) -> Result<HttpResponse> {
    let mut projets_map = projets.lock().unwrap();
    
    match projets_map.remove(&id) {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Projet non trouvé"
        })))
    }
}
