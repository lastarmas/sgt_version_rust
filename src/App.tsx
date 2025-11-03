import React from 'react';
import Header from './components/Header';
import Sidebar from './components/Sidebar';
import ProjectGrid from './components/ProjectGrid';
import { Plus } from 'lucide-react';

function App() {
  return (
    <div className="min-h-screen bg-gray-50">
      <Header />
      
      <div className="flex">
        <Sidebar />
        
        <main className="flex-1 p-8">
          <div className="max-w-7xl mx-auto">
            <div className="flex justify-between items-center mb-8">
              <div>
                <h1 className="text-3xl font-bold text-gray-900">Projets</h1>
                <p className="text-gray-600 mt-2">
                  Gestion et suivi de tous les projets de travaux techniques
                </p>
              </div>
              
              <button className="btn-primary flex items-center space-x-2">
                <Plus className="h-5 w-5" />
                <span>Nouveau Projet</span>
              </button>
            </div>
            
            <div className="mb-6 flex space-x-4">
              <button className="px-4 py-2 bg-primary-600 text-white rounded-lg font-medium">
                Tous
              </button>
              <button className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg font-medium">
                En cours
              </button>
              <button className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg font-medium">
                Planifiés
              </button>
              <button className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg font-medium">
                Terminés
              </button>
            </div>
            
            <ProjectGrid />
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;
