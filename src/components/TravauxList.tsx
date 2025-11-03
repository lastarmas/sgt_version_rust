import React, { useState } from 'react';
import { Plus, Search, Filter, Calendar } from 'lucide-react';
import { travaux, projets, utilisateurs } from '../data/mockData';
import { Travail } from '../types';

const TravauxList: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [filterStatut, setFilterStatut] = useState<string>('tous');
  const [filterType, setFilterType] = useState<string>('tous');

  const filteredTravaux = travaux.filter(travail => {
    const matchesSearch = travail.description.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesStatut = filterStatut === 'tous' || travail.statut === filterStatut;
    const matchesType = filterType === 'tous' || travail.type === filterType;
    return matchesSearch && matchesStatut && matchesType;
  });

  const getProjetNom = (projetId: string) => {
    return projets.find(p => p.id === projetId)?.nom || 'Projet inconnu';
  };

  const getResponsableNom = (responsableId: string) => {
    return utilisateurs.find(u => u.id === responsableId)?.nom || 'Responsable inconnu';
  };

  const getStatutColor = (statut: string) => {
    switch (statut) {
      case 'terminé': return 'bg-green-100 text-green-800';
      case 'en_cours': return 'bg-orange-100 text-orange-800';
      case 'à_faire': return 'bg-blue-100 text-blue-800';
      case 'échoué': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'migration': return 'bg-purple-100 text-purple-800';
      case 'clonebd': return 'bg-indigo-100 text-indigo-800';
      case 'rehausement': return 'bg-pink-100 text-pink-800';
      case 'installation': return 'bg-cyan-100 text-cyan-800';
      case 'maintenance': return 'bg-teal-100 text-teal-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getProgress = (checklist: any[]) => {
    const completed = checklist.filter(item => item.statut === 'terminé').length;
    return Math.round((completed / checklist.length) * 100);
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold text-gray-900">Suivi des Travaux</h1>
        <button className="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center space-x-2 hover:bg-blue-700 transition-colors">
          <Plus size={20} />
          <span>Nouveau Travail</span>
        </button>
      </div>

      {/* Filtres et recherche */}
      <div className="bg-white p-6 rounded-xl shadow-sm border">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" size={20} />
            <input
              type="text"
              placeholder="Rechercher un travail..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
          <div className="flex items-center space-x-2">
            <Filter size={20} className="text-gray-400" />
            <select
              value={filterStatut}
              onChange={(e) => setFilterStatut(e.target.value)}
              className="w-full border border-gray-300 rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="tous">Tous les statuts</option>
              <option value="à_faire">À faire</option>
              <option value="en_cours">En cours</option>
              <option value="terminé">Terminé</option>
              <option value="échoué">Échoué</option>
            </select>
          </div>
          <div className="flex items-center space-x-2">
            <Filter size={20} className="text-gray-400" />
            <select
              value={filterType}
              onChange={(e) => setFilterType(e.target.value)}
              className="w-full border border-gray-300 rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="tous">Tous les types</option>
              <option value="clonebd">Clone BD</option>
              <option value="migration">Migration</option>
              <option value="rehausement">Rehausement</option>
              <option value="installation">Installation</option>
              <option value="maintenance">Maintenance</option>
            </select>
          </div>
        </div>
      </div>

      {/* Liste des travaux */}
      <div className="space-y-4">
        {filteredTravaux.map((travail) => (
          <div key={travail.id} className="bg-white rounded-xl shadow-sm border p-6">
            <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between mb-4">
              <div className="flex-1">
                <div className="flex items-center space-x-4 mb-2">
                  <h3 className="text-lg font-semibold text-gray-900">{travail.description}</h3>
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatutColor(travail.statut)}`}>
                    {travail.statut.replace('_', ' ')}
                  </span>
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getTypeColor(travail.type)}`}>
                    {travail.type}
                  </span>
                </div>
                <div className="flex flex-wrap gap-4 text-sm text-gray-600">
                  <span>Projet: {getProjetNom(travail.projetId)}</span>
                  <span>Application: {travail.application.replace('_', ' ')}</span>
                  <span>Environnement: {travail.environnement}</span>
                  <span>Responsable: {getResponsableNom(travail.responsable)}</span>
                </div>
              </div>
              <div className="mt-4 lg:mt-0 lg:text-right">
                <div className="flex items-center space-x-2 text-sm text-gray-600">
                  <Calendar size={16} />
                  <span>{new Date(travail.dateDebut).toLocaleDateString('fr-FR')}</span>
                  <span>-</span>
                  <span>{new Date(travail.dateFin).toLocaleDateString('fr-FR')}</span>
                </div>
              </div>
            </div>

            {/* Checklist et progression */}
            <div className="border-t pt-4">
              <div className="flex justify-between items-center mb-3">
                <span className="text-sm font-medium text-gray-700">Checklist</span>
                <span className="text-sm text-gray-600">
                  {travail.checklist.filter(item => item.statut === 'terminé').length} / {travail.checklist.length} terminés
                </span>
              </div>
              
              {/* Barre de progression */}
              <div className="w-full bg-gray-200 rounded-full h-2 mb-4">
                <div 
                  className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                  style={{ width: `${getProgress(travail.checklist)}%` }}
                ></div>
              </div>

              {/* Items de checklist */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                {travail.checklist.map((item) => (
                  <div key={item.id} className="flex items-center space-x-3 p-2 border rounded-lg">
                    <div className={`w-3 h-3 rounded-full ${
                      item.statut === 'terminé' ? 'bg-green-500' :
                      item.statut === 'en_cours' ? 'bg-orange-500' :
                      'bg-gray-300'
                    }`}></div>
                    <span className={`text-sm ${
                      item.statut === 'terminé' ? 'text-green-700 line-through' :
                      item.statut === 'en_cours' ? 'text-orange-700' :
                      'text-gray-600'
                    }`}>
                      {item.description}
                    </span>
                  </div>
                ))}
              </div>
            </div>

            <div className="flex justify-end space-x-3 mt-4 pt-4 border-t">
              <button className="px-4 py-2 text-sm text-blue-600 hover:text-blue-800 font-medium">
                Détails
              </button>
              <button className="px-4 py-2 text-sm text-gray-600 hover:text-gray-800 font-medium">
                Éditer
              </button>
              <button className="px-4 py-2 text-sm bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-medium">
                Mettre à jour
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default TravauxList;
