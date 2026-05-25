import type { FC } from 'react';
import { Extension, ExtensionContext } from 'shared';
import AdminRegionsPage from './pages/AdminRegionsPage.tsx';
import ServerRegionCard from './components/ServerRegionCard.tsx';

class DevLuxxRegionExtension extends Extension {
  public cardConfigurationPage: FC | null = AdminRegionsPage;
  public cardComponent: FC | null = null;

  public initialize(ctx: ExtensionContext): void {
    ctx.extensionRegistry.pages.server.enterConsole((console) => {
      console.addStatCard(ServerRegionCard as FC);
    });
  }
}

export default new DevLuxxRegionExtension();
