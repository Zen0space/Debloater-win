import { useState } from 'react';
import {
  LayoutDashboard,
  Package,
  Shield,
  Settings,
  Sliders,
  Download,
  Cpu,
  Menu,
  X
} from 'lucide-react';
import Dashboard from './pages/dashboard/index';
import Apps from './pages/apps/index';
import Privacy from './pages/privacy/index';
import Services from './pages/services/index';
import Registry from './pages/registry/index';
import Updates from './pages/updates/index';
import System from './pages/system/index';

type Page = 'dashboard' | 'apps' | 'privacy' | 'services' | 'registry' | 'updates' | 'system';

const navItems = [
  { id: 'dashboard' as Page, label: 'Dashboard', icon: LayoutDashboard },
  { id: 'apps' as Page, label: 'Apps', icon: Package },
  { id: 'privacy' as Page, label: 'Privacy', icon: Shield },
  { id: 'services' as Page, label: 'Services', icon: Settings },
  { id: 'registry' as Page, label: 'Registry', icon: Sliders },
  { id: 'updates' as Page, label: 'Updates', icon: Download },
  { id: 'system' as Page, label: 'System', icon: Cpu },
];

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('dashboard');
  const [sidebarOpen, setSidebarOpen] = useState(true);

  const renderPage = () => {
    switch (currentPage) {
      case 'dashboard':
        return <Dashboard />;
      case 'apps':
        return <Apps />;
      case 'privacy':
        return <Privacy />;
      case 'services':
        return <Services />;
      case 'registry':
        return <Registry />;
      case 'updates':
        return <Updates />;
      case 'system':
        return <System />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <div className="flex h-screen bg-background text-foreground">
      <aside
        className={`${sidebarOpen ? 'w-64' : 'w-0'
          } transition-all duration-300 border-r border-border overflow-hidden`}
      >
        <div className="h-16 flex flex-col justify-center px-4 border-b border-border">
          <h1 className="text-xl font-bold leading-none">Debloater</h1>
          <p className="text-sm text-muted-foreground leading-none mt-1">Windows 11</p>
        </div>
        <nav className="p-4 space-y-2">
          {navItems.map((item) => {
            const Icon = item.icon;
            return (
              <button
                key={item.id}
                onClick={() => setCurrentPage(item.id)}
                className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg transition-colors ${currentPage === item.id
                  ? 'bg-primary text-primary-foreground'
                  : 'hover:bg-secondary text-muted-foreground hover:text-foreground'
                  }`}
              >
                <Icon size={18} />
                <span>{item.label}</span>
              </button>
            );
          })}
        </nav>
      </aside>

      <div className="flex-1 flex flex-col overflow-hidden min-w-[768px]">
        <header className="h-16 border-b border-border flex items-center justify-between px-4">
          <div className="flex items-center gap-2">
            <button
              onClick={() => setSidebarOpen(!sidebarOpen)}
              className="p-2 hover:bg-secondary rounded-lg transition-colors"
            >
              {sidebarOpen ? <X size={20} /> : <Menu size={20} />}
            </button>
          </div>
        </header>

        <main className="flex-1 overflow-auto p-6">
          {renderPage()}
        </main>
      </div>
    </div>
  );
}

export default App;
