import { Projet, Travail, Utilisateur } from '../types';

export const utilisateurs: Utilisateur[] = [
  {
    id: '1',
    nom: 'Jean Dupont',
    email: 'jean.dupont@entreprise.com',
    role: 'manager',
    equipe: 'Infrastructure'
  },
  {
    id: '2',
    nom: 'Marie Martin',
    email: 'marie.martin@entreprise.com',
    role: 'spécialiste',
    equipe: 'Base de données'
  },
  {
    id: '3',
    nom: 'Pierre Leroy',
    email: 'pierre.leroy@entreprise.com',
    role: 'conseiller',
    equipe: 'Applications'
  }
];

export const projets: Projet[] = [
  {
    id: 'PRJ001',
    code: 'PRJ001',
    nom: 'Migration Espresso GFR',
    description: 'Migration de la version 2.5 vers 3.0',
    dateDebut: '2024-01-15',
    dateFinPrevue: '2024-03-30',
    statut: 'en_cours',
    priorite: 'haute'
  },
  {
    id: 'PRJ002',
    code: 'PRJ002',
    nom: 'Rehausement Infrastructure GRM',
    description: 'Augmentation des capacités serveurs',
    dateDebut: '2024-02-01',
    dateFinPrevue: '2024-04-15',
    statut: 'planifié',
    priorite: 'moyenne'
  },
  {
    id: 'PRJ003',
    code: 'PRJ003',
    nom: 'Clone BD Production GRH',
    description: 'Création environnement test',
    dateDebut: '2024-01-20',
    dateFinPrevue: '2024-02-10',
    statut: 'terminé',
    priorite: 'faible'
  }
];

export const travaux: Travail[] = [
  {
    id: 'TRV001',
    projetId: 'PRJ001',
    type: 'migration',
    application: 'espresso_gfr',
    environnement: 'production',
    description: 'Migration base de données',
    dateDebut: '2024-01-15',
    dateFin: '2024-01-20',
    statut: 'terminé',
    responsable: '2',
    checklist: [
      {
        id: 'CHK001',
        description: 'Sauvegarde base de données',
        statut: 'terminé',
        ordre: 1
      },
      {
        id: 'CHK002',
        description: 'Validation schéma',
        statut: 'terminé',
        ordre: 2
      },
      {
        id: 'CHK003',
        description: 'Tests fonctionnels',
        statut: 'terminé',
        ordre: 3
      }
    ]
  },
  {
    id: 'TRV002',
    projetId: 'PRJ001',
    type: 'installation',
    application: 'espresso_gfr',
    environnement: 'test',
    description: 'Installation nouvelle version',
    dateDebut: '2024-02-01',
    dateFin: '2024-02-05',
    statut: 'en_cours',
    responsable: '2',
    checklist: [
      {
        id: 'CHK004',
        description: 'Préparation serveur',
        statut: 'terminé',
        ordre: 1
      },
      {
        id: 'CHK005',
        description: 'Installation packages',
        statut: 'en_cours',
        ordre: 2
      },
      {
        id: 'CHK006',
        description: 'Configuration',
        statut: 'à_faire',
        ordre: 3
      }
    ]
  },
  {
    id: 'TRV003',
    projetId: 'PRJ002',
    type: 'rehausement',
    application: 'espresso_grm',
    environnement: 'production',
    description: 'Augmentation RAM serveurs',
    dateDebut: '2024-02-10',
    dateFin: '2024-02-15',
    statut: 'à_faire',
    responsable: '1',
    checklist: [
      {
        id: 'CHK007',
        description: 'Commande matériel',
        statut: 'terminé',
        ordre: 1
      },
      {
        id: 'CHK008',
        description: 'Planification intervention',
        statut: 'à_faire',
        ordre: 2
      },
      {
        id: 'CHK009',
        description: 'Tests performance',
        statut: 'à_faire',
        ordre: 3
      }
    ]
  }
];
