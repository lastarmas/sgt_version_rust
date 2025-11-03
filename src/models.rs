use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projet {
    pub id: Uuid,
    pub code: String,
    pub nom: String,
    pub description: String,
    pub date_debut: DateTime<Utc>,
    pub date_fin_prevue: DateTime<Utc>,
    pub statut: StatutProjet,
    pub priorite: Priorite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Travail {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utilisateur {
    pub id: Uuid,
    pub nom: String,
    pub email: String,
    pub role: Role,
    pub equipe: String,
    pub actif: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: Uuid,
    pub travail_id: Uuid,
    pub description: String,
    pub statut: StatutChecklist,
    pub responsable: Uuid,
    pub date_echeance: Option<DateTime<Utc>>,
    pub commentaires: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatutProjet {
    Planifie,
    EnCours,
    Termine,
    Suspendu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatutTravail {
    Planifie,
    EnCours,
    Termine,
    Suspendu,
    Annule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatutChecklist {
    NonDemarre,
    EnCours,
    Termine,
    Bloque,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeTravail {
    CloneBd,
    Migration,
    Rehausement,
    MajApplication,
    Autre,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Application {
    EspressoGfr,
    EspressoGrm,
    EspressoGrh,
    EspressoGpa,
    Autre,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environnement {
    Test,
    Formation,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priorite {
    Faible,
    Moyenne,
    Haute,
    Critique,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Conseiller,
    Manager,
    Specialiste,
    Admin,
}
