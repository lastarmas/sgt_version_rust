use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::Mutex;
use std::collections::HashMap;

mod models;
mod handlers;
mod database;

use models::{Projet, Travail, Utilisateur, ChecklistItem};
use handlers::{projet_handlers, travail_handlers, utilisateur_handlers};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Démarrage du serveur Rust/Actix sur http://localhost:8080");

    // Données mock en mémoire (remplacera par base de données)
    let projets_data = web::Data::new(Mutex::new(HashMap::<Uuid, Projet>::new()));
    let travaux_data = web::Data::new(Mutex::new(HashMap::<Uuid, Travail>::new()));
    let utilisateurs_data = web::Data::new(Mutex::new(HashMap::<Uuid, Utilisateur>::new()));

    // Initialisation avec des données de test
    init_mock_data(&projets_data, &travaux_data, &utilisateurs_data);

    HttpServer::new(move || {
        App::new()
            .app_data(projets_data.clone())
            .app_data(travaux_data.clone())
            .app_data(utilisateurs_data.clone())
            .configure(projet_handlers::config)
            .configure(travail_handlers::config)
            .configure(utilisateur_handlers::config)
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Serveur Rust/Actix fonctionnel",
        "timestamp": Utc::now().to_rfc3339()
    })))
}

fn init_mock_data(
    projets: &web::Data<Mutex<HashMap<Uuid, Projet>>>,
    travaux: &web::Data<Mutex<HashMap<Uuid, Travail>>>,
    utilisateurs: &web::Data<Mutex<HashMap<Uuid, Utilisateur>>>,
) {
    let mut projets_map = projets.lock().unwrap();
    let mut travaux_map = travaux.lock().unwrap();
    let mut utilisateurs_map = utilisateurs.lock().unwrap();

    // Utilisateurs mock
    let user1 = Utilisateur {
        id: Uuid::new_v4(),
        nom: "Jean Dupont".to_string(),
        email: "jean.dupont@entreprise.com".to_string(),
        role: models::Role::Manager,
        equipe: "Infrastructure".to_string(),
        actif: true,
    };

    let user2 = Utilisateur {
        id: Uuid::new_v4(),
        nom: "Marie Martin".to_string(),
        email: "marie.martin@entreprise.com".to_string(),
        role: models::Role::Specialiste,
        equipe: "Base de données".to_string(),
        actif: true,
    };

    utilisateurs_map.insert(user1.id, user1.clone());
    utilisateurs_map.insert(user2.id, user2.clone());

    // Projets mock
    let projet1 = Projet {
        id: Uuid::new_v4(),
        code: "PRJ001".to_string(),
        nom: "Migration Espresso GFR".to_string(),
        description: "Migration de la version 2.5 vers 3.0".to_string(),
        date_debut: Utc::now(),
        date_fin_prevue: Utc::now() + chrono::Duration::days(60),
        statut: models::StatutProjet::EnCours,
        priorite: models::Priorite::Haute,
    };

    let projet2 = Projet {
        id: Uuid::new_v4(),
        code: "PRJ002".to_string(),
        nom: "Rehausement Infrastructure GRM".to_string(),
        description: "Augmentation des capacités serveurs".to_string(),
        date_debut: Utc::now() + chrono::Duration::days(5),
        date_fin_prevue: Utc::now() + chrono::Duration::days(45),
        statut: models::StatutProjet::Planifie,
        priorite: models::Priorite::Moyenne,
    };

    projets_map.insert(projet1.id, projet1.clone());
    projets_map.insert(projet2.id, projet2.clone());

    // Travaux mock
    let travail1 = Travail {
        id: Uuid::new_v4(),
        projet_id: projet1.id,
        type_travail: models::TypeTravail::Migration,
        application: models::Application::EspressoGfr,
        environnement: models::Environnement::Production,
        description: "Migration base de données".to_string(),
        date_debut: Utc::now(),
        date_fin_prevue: Utc::now() + chrono::Duration::days(5),
        statut: models::StatutTravail::EnCours,
        responsable: user2.id,
        equipe: vec![user1.id, user2.id],
    };

    travaux_map.insert(travail1.id, travail1);
}
