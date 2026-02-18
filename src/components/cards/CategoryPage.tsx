import { useState } from 'react';
import { Search } from 'lucide-react';
import { useDebloatItems } from '../../hooks/useDebloatItems';
import { useAppStore } from '../../store/useAppStore';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../components/ui/card';
import { Switch } from '../../components/ui/switch';
import { cn } from '../../lib/utils';

interface CategoryPageProps {
  title: string;
  description: string;
  category: string;
}

export function CategoryPage({ title, description, category }: CategoryPageProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const { items, loading, error } = useDebloatItems(category);
  const { selectedItems, toggleItem, selectAll, deselectAll } = useAppStore();

  const filteredItems = items.filter(item =>
    item.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    item.description.toLowerCase().includes(searchQuery.toLowerCase())
  );

  if (loading) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-3xl font-bold mb-2">{title}</h1>
          <p className="text-muted-foreground">{description}</p>
        </div>
        <div className="bg-card border border-border rounded-lg p-6">
          <p className="text-muted-foreground">Loading data...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-3xl font-bold mb-2">{title}</h1>
          <p className="text-muted-foreground">{description}</p>
        </div>
        <div className="bg-destructive/10 border border-destructive/20 rounded-lg p-6">
          <p className="text-destructive font-medium">Error loading data</p>
          <p className="text-muted-foreground mt-1">{error}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold mb-2">{title}</h1>
        <p className="text-muted-foreground">{description}</p>
      </div>

      <div className="space-y-4">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground" size={18} />
          <input
            type="text"
            placeholder="Search items..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-10 pr-4 py-2 bg-card border border-border rounded-lg focus:outline-none focus:ring-2 focus:ring-ring"
          />
        </div>

        <div className="flex items-center justify-between">
          <p className="text-sm text-muted-foreground">
            {filteredItems.length} item{filteredItems.length !== 1 ? 's' : ''} found
          </p>
          <div className="flex gap-2">
            <button
              onClick={() => selectAll(filteredItems)}
              className="text-sm text-primary hover:underline"
            >
              Select All
            </button>
            <button
              onClick={() => deselectAll()}
              className="text-sm text-primary hover:underline"
            >
              Deselect All
            </button>
          </div>
        </div>

        <div className="grid gap-4">
          {filteredItems.map((item) => (
            <Card
              key={item.id}
              className={cn(
                'transition-colors hover:bg-accent',
                selectedItems.has(item.id) && 'border-primary'
              )}
            >
              <CardHeader className="flex flex-row items-start justify-between space-y-0 pb-2">
                <div className="space-y-1 flex-1">
                  <CardTitle className="text-lg">{item.name}</CardTitle>
                  <CardDescription>{item.description}</CardDescription>
                </div>
                <Switch
                  checked={selectedItems.has(item.id)}
                  onCheckedChange={() => toggleItem(item.id)}
                />
              </CardHeader>
              <CardContent className="pt-0">
                <div className="flex items-center gap-2">
                  <span
                    className={cn(
                      'text-xs px-2 py-1 rounded-full',
                      item.safe ? 'bg-green-500/10 text-green-500' : 'bg-yellow-500/10 text-yellow-500'
                    )}
                  >
                    {item.safe ? 'Safe' : 'Use with caution'}
                  </span>
                  {item.isInstalled !== undefined && (
                    <span
                      className={cn(
                        'text-xs px-2 py-1 rounded-full',
                        item.isInstalled ? 'bg-blue-500/10 text-blue-500' : 'bg-gray-500/10 text-gray-500'
                      )}
                    >
                      {item.isInstalled ? 'Installed' : 'Not installed'}
                    </span>
                  )}
                  <span className="text-xs text-muted-foreground">
                    {item.category}
                  </span>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>

        {filteredItems.length === 0 && (
          <div className="text-center py-12">
            <p className="text-muted-foreground">No items found matching your search.</p>
          </div>
        )}
      </div>
    </div>
  );
}
