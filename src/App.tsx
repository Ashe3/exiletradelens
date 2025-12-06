import { Tabs } from '@base-ui-components/react/tabs';
import './App.css';
import { Header } from './components/Header';
import { Sidebar } from './components/Sidebar';
import { useState } from 'react';
import { Content } from './components/Content';

type TabValue = 'latest' | 'history';

const App = () => {
  const [activeTab, setActiveTab] = useState<TabValue>('latest');

  return (
    <div className="flex flex-col h-screen">
      <Header />
      <Tabs.Root
        value={activeTab}
        onValueChange={setActiveTab}
        orientation="vertical"
        className="flex flex-1"
      >
        <Sidebar>
          <Tabs.List className="flex flex-col gap-4 ">
            <Tabs.Tab
              value={'latest'}
              className="px-16 py-4 transition-colors cursor-pointer text-gray-700 data-active:bg-gray-300 data-active:text-blue-600 hover:bg-gray-200 hover:text-blue-500"
            >
              Latest search
            </Tabs.Tab>
            <Tabs.Tab
              value={'history'}
              className="px-16 py-4 transition-colors cursor-pointer text-gray-700 data-active:bg-gray-300 data-active:text-blue-600 hover:bg-gray-200 hover:text-blue-500"
            >
              Search history
            </Tabs.Tab>
            <Tabs.Indicator />
          </Tabs.List>
        </Sidebar>
        <Content>
          <Tabs.Panel value="latest">
            <div>latest search</div>
          </Tabs.Panel>
          <Tabs.Panel value="history">
            <div>search history</div>
          </Tabs.Panel>
        </Content>
      </Tabs.Root>
    </div>
  );
};

export default App;
