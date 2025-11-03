import React from 'react';
import { Calendar, AlertTriangle, CheckCircle, Clock } from 'lucide-react';
import { projets, travaux } from '../data/mockData';

const Dashboard: React.FC = () => {
  const stats = {
    totalProjets: projets.length,
    projetsEnCours: projets.filter(p => p.statut === 'en_cours').length,
    travauxEnRetard: travaux.filter(t => new Date(t.dateFin) < new Date() && t.statut !== 'terminé').length,
    travauxTermines: travaux.filter(t => t.statut === 'terminé').length
  };

  const travauxRecents = travaux
    .sort((a, b) => new Date(b.dateDebut).getTime() - new Date(a.dateDebut).getTime())
    .slice(0, 5);

  return (
    <div className="space-y-8">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold text-gray-900">Tableau de bord</h1>
        <div className="flex items-center space-x-2 text-gray-600">
          <Calendar size={20} />
          <span>{new Date().toLocaleDateString('fr-FR')}</span>
        </div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="bg-white p-6 rounded-xl shadow-sm border">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">Total Projets</p>
              <p className="text-2xl font-bold text-gray-900">{stats.totalProjets}</p>
            </div>
            <div className="p-3 bg-blue-100 rounded-lg">
              <ClipboardList className="text-blue-600" size={24} />
            </div>
          </div>
        </div>

        <div className="bg-white p-6 rounded-xl shadow-sm border">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">Projets en cours</p>
              <p className="text-2xl font-bold text-orange-600">{stats.projetsEnCours}</p>
            </div>
            <div className="p-3 bg-orange-100 rounded-lg">
              <Clock className="text-orange-600" size={24} />
            </div>
          </div>
        </div>

        <div className="bg-white p-6 rounded-xl shadow-sm border">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">Travaux en retard</p>
              <p className="text-2xl font-bold text-red-600">{stats.travauxEnRetard}</p>
            </div>
            <div className="p-3 bg-red-100 rounded-lg">
              <AlertTriangle className="text-red-600" size={24} />
            </div>
          </div>
        </div>

        <div className="bg-white p-6 rounded-xl shadow-sm border">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">Travaux terminés</p>
              <p className="text-2xl font-bold text-green-600">{stats.travauxTermines}</p>
            </div>
            <div className="p-3 bg-green-100 rounded-lg">
              <CheckCircle className="text-green-600" size={24} />
            </div>
          </div>
        </div>
      </div>

      {/* Travaux récents */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <div className="bg-white rounded-xl shadow-sm border p-6">
          <h2 className="text-xl font-semibold text-gray-900 mb-4">Travaux récents</h2>
          <div className="space-y-4">
            {travauxRecents.map((travail) => (
              <div key={travail.id} className="flex items-center justify-between p-4 border rounded-lg">
                <div>
                  <p className="font-medium text-gray-900">{travail.description}</p>
                  <p className="text-sm text-gray-600">
                    {travail.application} • {travail.environnement}
                  </p>
                </div>
                <span className={`px-3 py-1 rounded-full text-xs font-medium ${
                  travail.statut === 'terminé' ? 'bg-green-100 text-green-800' :
                  travail.statut === 'en_cours' ? 'bg-orange-100 text-orange-800' :
                  'bg-gray-100 text-gray-800'
                }`}>
                  {travail.statut.replace('_', ' ')}
                </span>
              </div>
            ))}
          </div>
        </div>

        {/* Projets en cours */}
        <div className="bg-white rounded-xl shadow-sm border p-6">
          <h2 className="text-xl font-semibold text-gray-900 mb-4">Projets en cours</h2>
          <div className="space-y-4">
            {projets.filter(p => p.statut === 'en_cours').map((projet) => (
              <div key={projet.id} className="p-4 border rounded-lg">
                <div className="flex justify-between items-start mb-2">
                  <h3 className="font-semibold text-gray-900">{projet.nom}</h3>
                  <span className={`px-2 py-1 rounded text-xs font-medium ${
                    projet.priorite === 'critique' ? 'bg-red-100 text-red-800' :
                    projet.priorite === 'haute' ? 'bg-orange-100 text-orange-800' :
                    'bg-blue-100 text-blue-800'
                  }`}>
                    {projet.priorite}
                  </span>
                </div>
                <p className="text-sm text-gray-600 mb-3">{projet.description}</p>
                <div className="flex justify-between text-xs text-gray-500">
                  <span>Début: {new Date(projet.dateDebut).toLocaleDateString('fr-FR')}</span>
                  <span>Fin: {new Date(projet.dateFinPrevue).toLocaleDateString('fr-FR')}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

// Add missing icon component
const ClipboardList = ({ className, size }: { className?: string; size: number }) => (
  <svg className={className} width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor">
    <path d="M9 5H7a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2M9 5a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2M9 5a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2"/>
  </svg>
);

export default Dashboard;
