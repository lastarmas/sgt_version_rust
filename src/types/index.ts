export interface Projet {
  id: string;
  code: string;
  nom: string;
  description: string;
  dateDebut: string;
  dateFinPrevue: string;
  statut: 'planifié' | 'en_cours' | 'terminé' | 'suspendu';
  priorite: 'faible' | 'moyenne' | 'élevée' | 'critique';
}

export interface Travail {
  id: string;
  projetId: string;
  type: 'clonebd' | 'migration' | 'rehausement' | 'maj_application' | 'autre';
  application: 'espresso_gfr' | 'espresso_grm' | 'espresso_grh' | 'espresso_gpa' | 'autre';
  environnement: 'test' | 'formation' | 'production';
  description: string;
  dateDebut: string;
  dateFinPrevue: string;
  statut: 'planifié' | 'en_cours' | 'terminé' | 'suspendu' | 'annulé';
  responsable: string;
  equipe: string[];
}

export interface ChecklistItem {
  id: string;
  travailId: string;
  description: string;
  statut: 'non_démarré' | 'en_cours' | 'terminé' | 'bloqué';
  responsable: string;
  dateEcheance?: string;
  commentaires?: string;
}

export interface Utilisateur {
  id: string;
  nom: string;
  email: string;
  role: 'conseiller' | 'manager' | 'specialiste' | 'admin';
  equipe: string;
  actif: boolean;
}
