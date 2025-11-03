import React from 'react';
import { Folder, Settings, Users, BarChart3, Calendar } from 'lucide-react';

const Sidebar: React.FC = () => {
  const menuItems = [
    { icon: Folder, label: 'Projets', active: true },
    { icon: Calendar, label: 'Travaux', active: false },
    { icon: Users, label: 'Équipes', active: false },
    { icon: BarChart3, label: 'Rapports', active: false },
    { icon: Settings, label: 'Paramètres', active: false },
  ];

  return (
    <aside className="w-64 bg-white shadow-sm border-r border-gray-200">
      <nav className="h-full flex flex-col">
        <div className="flex-1 px-4 py-6">
          <ul className="space-y-2">
            {menuItems.map((item) => {
              const Icon = item.icon;
              return (
                <li key={item.label}>
                  <a
                    href="#"
                    className={`flex items-center px-3 py-2 text-sm font-medium rounded-lg transition-colors ${
                      item.active
                        ? 'bg-primary-50 text-primary-700 border-r-2 border-primary-700'
                        : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
                    }`}
                  >
                    <Icon className="h-5 w-5 mr-3" />
                    {item.label}
                  </a>
                </li>
              );
            })}
          </ul>
        </div>
        
        <div className="px-4 py-4 border-t border-gray-200">
          <div className="bg-primary-50 rounded-lg p-4">
            <h3 className="text-sm font-medium text-primary-900 mb-1">
              Support
            </h3>
            <p className="text-xs text-primary-700">
              Besoin d'aide ? Contactez notre équipe.
            </p>
          </div>
        </div>
      </nav>
    </aside>
  );
};

export default Sidebar;
