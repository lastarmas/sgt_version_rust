use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::{Utilisateur, Role};

#[derive(Debug, Deserialize)]
pub struct CreateUtilisateurRequest {
    pub nom: String,
    pub email: String,
    pub role: Role,
    pub equipe: String,
    pub actif: bool,
}

#[derive(Debug, Serialize)]
pub struct UtilisateurResponse {
    pub id: Uuid,
    pub nom: String,
    pub email: String,
    pub role: Role,
    pub equipe: String,
    pub actif: bool,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/utilisateurs")
            .route("", web::get().to(get_utilisateurs))
            .route("", web::post().to(create_utilisateur))
            .route("/{id}", web::get().to(get_utilisateur))
            .route("/{id}", web::put().to(update_utilisateur))
            .route("/{id}", web::delete().to(delete_utilisateur))
    );
}

pub async fn get_utilisateurs(utilisateurs: web::Data<Mutex<HashMap<Uuid, Utilisateur>>>) -> Result<HttpResponse> {
    let utilisateurs_map = utilisateurs.lock().unwrap();
    let utilisateurs_list: Vec<UtilisateurResponse> = utilisateurs_map.values()
        .map(|u| UtilisateurResponse {
            id: u.id,
            nom: u.nom.clone(),
            email: u.email.clone(),
            role: u.role.clone(),
            equipe: u.equipe.clone(),
            actif: u.actif,
        })
        .collect();

    Ok(HttpResponse::Ok().json(utilisateurs_list))
}

pub async fn get_utilisateur(
    web::Path(id): web::Path<Uuid>,
    utilisateurs: web::Data<Mutex<HashMap<Uuid, Utilisateur>>>,
) -> Result<HttpResponse> {
    let utilisateurs_map = utilisateurs.lock().unwrap();
    
    match utilisateurs_map.get(&id) {
        Some(utilisateur) => {
            let response = UtilisateurResponse {
                id: utilisateur.id,
                nom: utilisateur.nom.clone(),
                email: utilisateur.email.clone(),
                role: utilisateur.role.clone(),
                equipe: utilisateur.equipe.clone(),
                actif: utilisateur.actif,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Utilisateur non trouvé"
        })))
    }
}

pub async fn create_utilisateur(
    utilisateur_req: web::Json<CreateUtilisateurRequest>,
    utilisateurs: web::Data<Mutex<HashMap<Uuid, Utilisateur>>>,
) -> Result<HttpResponse> {
    let mut utilisateurs_map = utilisateurs.lock().unwrap();
    
    let utilisateur = Utilisateur {
        id: Uuid::new_v4(),
        nom: utilisateur_req.nom.clone(),
        email: utilisateur_req.email.clone(),
        role: utilisateur_req.role.clone(),
        equipe: utilisateur_req.equipe.clone(),
        actif: utilisateur_req.actif,
    };

    utilisateurs_map.insert(utilisateur.id, utilisateur.clone());

    let response = UtilisateurResponse {
        id: utilisateur.id,
        nom: utilisateur.nom,
        email: utilisateur.email,
        role: utilisateur.role,
        equipe: utilisateur.equipe,
        actif: utilisateur.actif,
    };

    Ok(HttpResponse::Created().json(response))
}

pub async fn update_utilisateur(
    web::Path(id): web::Path<Uuid>,
    utilisateur_req: web::Json<CreateUtilisateurRequest>,
    utilisateurs: web::Data<Mutex<HashMap<Uuid, Utilisateur>>>,
) -> Result<HttpResponse> {
    let mut utilisateurs_map = utilisateurs.lock().unwrap();
    
    if let Some(existing_utilisateur) = utilisateurs_map.get_mut(&id) {
        existing_utilisateur.nom = utilisateur_req.nom.clone();
        existing_utilisateur.email = utilisateur_req.email.clone();
        existing_utilisateur.role = utilisateur_req.role.clone();
        existing_utilisateur.equipe = utilisateur_req.equipe.clone();
        existing_utilisateur.actif = utilisateur_req.actif;

        let response = UtilisateurResponse {
            id: existing_utilisateur.id,
            nom: existing_utilisateur.nom.clone(),
            email: existing_utilisateur.email.clone(),
            role: existing_utilisateur.role.clone(),
            equipe: existing_utilisateur.equipe.clone(),
            actif: existing_utilisateur.actif,
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Utilisateur non trouvé"
        })))
    }
}

pub async fn delete_utilisateur(
    web::Path(id): web::Path<Uuid>,
    utilisateurs: web::Data<Mutex<HashMap<Uuid, Utilisateur>>>,
) -> Result<HttpResponse> {
    let mut utilisateurs_map = utilisateurs.lock().unwrap();
    
    match utilisateurs_map.remove(&id) {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Utilisateur non trouvé"
        })))
    }
}
