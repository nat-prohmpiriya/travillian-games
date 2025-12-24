<script lang="ts">
  import { Card } from '$lib/components/ui/card';

  export type BuildingType =
    | 'empty'
    | 'main_building'
    | 'warehouse'
    | 'granary'
    | 'barracks'
    | 'stable'
    | 'workshop'
    | 'academy'
    | 'smithy'
    | 'market'
    | 'embassy'
    | 'palace'
    | 'residence'
    | 'wall'
    | 'rally_point'
    | 'cranny'
    | 'hero_mansion'
    | 'tavern'
    | 'town_hall'
    | 'treasury'
    | 'trade_office'
    | 'woodcutter'
    | 'clay_pit'
    | 'iron_mine'
    | 'crop_field';

  interface Building {
    id: string;
    type: BuildingType;
    level: number;
    isUpgrading: boolean;
    upgradeEndsAt?: Date;
  }

  interface Props {
    building: Building | null;
    slot: number;
    isResourceField?: boolean;
    onclick?: () => void;
  }

  let { building, slot, isResourceField = false, onclick }: Props = $props();

  const buildingIcons: Record<BuildingType, string> = {
    empty: '➕',
    main_building: '🏛️',
    warehouse: '📦',
    granary: '🌾',
    barracks: '⚔️',
    stable: '🐎',
    workshop: '🔧',
    academy: '📚',
    smithy: '🔨',
    market: '🏪',
    embassy: '🏰',
    palace: '👑',
    residence: '🏠',
    wall: '🧱',
    rally_point: '🚩',
    cranny: '🕳️',
    hero_mansion: '🦸',
    tavern: '🍺',
    town_hall: '🏛️',
    treasury: '💰',
    trade_office: '📊',
    woodcutter: '🪵',
    clay_pit: '🧱',
    iron_mine: '⛏️',
    crop_field: '🌾'
  };

  const buildingNames: Record<BuildingType, string> = {
    empty: 'Empty',
    main_building: 'Main Building',
    warehouse: 'Warehouse',
    granary: 'Granary',
    barracks: 'Barracks',
    stable: 'Stable',
    workshop: 'Workshop',
    academy: 'Academy',
    smithy: 'Smithy',
    market: 'Market',
    embassy: 'Embassy',
    palace: 'Palace',
    residence: 'Residence',
    wall: 'Wall',
    rally_point: 'Rally Point',
    cranny: 'Cranny',
    hero_mansion: 'Hero Mansion',
    tavern: 'Tavern',
    town_hall: 'Town Hall',
    treasury: 'Treasury',
    trade_office: 'Trade Office',
    woodcutter: 'Woodcutter',
    clay_pit: 'Clay Pit',
    iron_mine: 'Iron Mine',
    crop_field: 'Crop Field'
  };

  const isEmpty = $derived(!building || building.type === 'empty');
  const icon = $derived(building ? buildingIcons[building.type] : '➕');
  const name = $derived(building ? buildingNames[building.type] : 'Build');
</script>

<button
  type="button"
  {onclick}
  class="w-full aspect-square focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 rounded-xl"
>
  <Card
    class="h-full flex flex-col items-center justify-center gap-1 transition-all hover:shadow-md hover:-translate-y-0.5
           {isEmpty ? 'border-dashed bg-muted/30 hover:bg-muted/50' : ''}
           {building?.isUpgrading ? 'ring-2 ring-amber-500' : ''}
           {isResourceField ? 'bg-emerald-50 dark:bg-emerald-950/20' : ''}"
  >
    <!-- Building Icon -->
    <span class="text-2xl md:text-3xl {isEmpty ? 'opacity-50' : ''}">
      {icon}
    </span>

    <!-- Building Name -->
    <span class="text-xs font-medium text-center px-1 truncate w-full {isEmpty ? 'text-muted-foreground' : ''}">
      {name}
    </span>

    <!-- Level Badge -->
    {#if building && !isEmpty}
      <span class="text-xs font-bold bg-primary text-primary-foreground px-2 py-0.5 rounded-full">
        Lv.{building.level}
      </span>
    {/if}

    <!-- Upgrading Indicator -->
    {#if building?.isUpgrading}
      <div class="absolute top-1 right-1">
        <span class="flex h-3 w-3">
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75"></span>
          <span class="relative inline-flex rounded-full h-3 w-3 bg-amber-500"></span>
        </span>
      </div>
    {/if}
  </Card>
</button>
