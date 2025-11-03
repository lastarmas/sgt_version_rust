// Module pour la gestion de la base de données
// À implémenter avec SQLx pour PostgreSQL

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use anyhow::Result;

pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Database { pool })
    }

    pub async fn init_schema(&self) -> Result<()> {
        // Création des tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS projets (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                code VARCHAR(50) UNIQUE NOT NULL,
                nom VARCHAR(255) NOT NULL,
                description TEXT,
                date_debut TIMESTAMPTZ NOT NULL,
                date_fin_prevue TIMESTAMPTZ NOT NULL,
                statut VARCHAR(20) NOT NULL,
                priorite VARCHAR(20) NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            )
            "#
        ).execute(&self.pool).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS utilisateurs (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                nom VARCHAR(255) NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                role VARCHAR(20) NOT NULL,
                equipe VARCHAR(100) NOT NULL,
                actif BOOLEAN DEFAULT TRUE,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            )
            "#
        ).execute(&self.pool).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS travaux (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                projet_id UUID REFERENCES projets(id),
                type_travail VARCHAR(20) NOT NULL,
                application VARCHAR(20) NOT NULL,
                environnement VARCHAR(20) NOT NULL,
                description TEXT NOT NULL,
                date_debut TIMESTAMPTZ NOT NULL,
                date_fin_prevue TIMESTAMPTZ NOT NULL,
                statut VARCHAR(20) NOT NULL,
                responsable UUID REFERENCES utilisateurs(id),
                equipe UUID[],
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            )
            "#
        ).execute(&self.pool).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS checklist_items (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                travail_id UUID REFERENCES travaux(id),
                description TEXT NOT NULL,
                statut VARCHAR(20) NOT NULL,
                responsable UUID REFERENCES utilisateurs(id),
                date_echeance TIMESTAMPTZ,
                commentaires TEXT,
                ordre INTEGER NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            )
            "#
        ).execute(&self.pool).await?;

        Ok(())
    }
}
