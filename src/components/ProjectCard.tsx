import React from 'react';
import { Calendar, Users, AlertCircle } from 'lucide-react';
import { Projet } from '../types';

interface ProjectCardProps {
  projet: Projet;
}

const ProjectCard: React.FC<ProjectCardProps> = ({ projet }) => {
  const getStatusColor = (statut: string) => {
    switch (statut) {
      case 'planifié':
        return 'bg-blue-100 text-blue-800';
      case 'en_cours':
        return 'bg-green-100 text-green-800';
      case 'terminé':
        return 'bg-gray-100 text-gray-800';
      case 'suspendu':
        return 'bg-yellow-100 text-yellow-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const getPriorityIcon = (priorite: string) => {
    if (priorite === 'critique' || priorite === 'élevée') {
      return <AlertCircle className="h-4 w-4" />;
    }
    return null;
  };

  return (
    <div className="card hover:shadow-md transition-shadow duration-200">
      <div className="flex justify-between items-start mb-4">
        <div>
          <h3 className="text-lg font-semibold text-gray-900">{projet.code}</h3>
          <p className="text-sm text-gray-600 mt-1">{projet.nom}</p>
        </div>
        <div className="flex items-center space-x-2">
          {getPriorityIcon(projet.priorite)}
          <span className={`px-2 py-1 text-xs font-medium rounded-full ${getStatusColor(projet.statut)}`}>
            {projet.statut}
          </span>
        </div>
      </div>
      
      <p className="text-sm text-gray-600 mb-4 line-clamp-2">{projet.description}</p>
      
      <div className="flex items-center justify-between text-sm text-gray-500">
        <div className="flex items-center space-x-4">
          <div className="flex items-center">
            <Calendar className="h-4 w-4 mr-1" />
            <span>{new Date(projet.dateFinPrevue).toLocaleDateString()}</span>
          </div>
          <div className="flex items-center">
            <Users className="h-4 w-4 mr-1" />
            <span>3</span>
          </div>
        </div>
        
        <span className={`px-2 py-1 text-xs font-medium rounded ${
          projet.priorite === 'critique' ? 'bg-red-100 text-red-800' :
          projet.priorite === 'élevée' ? 'bg-orange-100 text-orange-800' :
          projet.priorite === 'moyenne' ? 'bg-yellow-100 text-yellow-800' :
          'bg-green-100 text-green-800'
        }`}>
          {projet.priorite}
        </span>
      </div>
    </div>
  );
};

export default ProjectCard;
