import React from 'react';
import ProjectCard from './ProjectCard';
import { Projet } from '../types';

const mockProjects: Projet[] = [
  {
    id: '1',
    code: 'PRJ2024001',
    nom: 'Migration Espresso GFR',
    description: 'Migration de la base de données Espresso GFR vers le nouveau serveur avec optimisation des performances.',
    dateDebut: '2024-01-15',
    dateFinPrevue: '2024-03-20',
    statut: 'en_cours',
    priorite: 'élevée',
  },
  {
    id: '2',
    code: 'PRJ2024002',
    nom: 'Rehausement GRM Test',
    description: 'Rehausement de version pour environnement de test GRM avec validation des nouvelles fonctionnalités.',
    dateDebut: '2024-02-01',
    dateFinPrevue: '2024-02-28',
    statut: 'planifié',
    priorite: 'moyenne',
  },
  {
    id: '3',
    code: 'PRJ2024003',
    nom: 'Clone BD Production GRH',
    description: 'Création d\'un clone de la base de données GRH production pour tests de performance.',
    dateDebut: '2024-01-20',
    dateFinPrevue: '2024-01-25',
    statut: 'terminé',
    priorite: 'faible',
  },
  {
    id: '4',
    code: 'PRJ2024004',
    nom: 'Mise à jour GPA Formation',
    description: 'Application des correctifs et mises à jour de sécurité sur l\'environnement de formation GPA.',
    dateDebut: '2024-03-01',
    dateFinPrevue: '2024-03-15',
    statut: 'planifié',
    priorite: 'critique',
  },
];

const ProjectGrid: React.FC = () => {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
      {mockProjects.map((projet) => (
        <ProjectCard key={projet.id} projet={projet} />
      ))}
    </div>
  );
};

export default ProjectGrid;
