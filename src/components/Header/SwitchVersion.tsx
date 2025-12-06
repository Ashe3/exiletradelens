import { Switch } from '@base-ui-components/react/switch';
import { useStore } from '../../store';
import { useCallback } from 'react';

export const SwitchVersion = () => {
  const { poeVersion, setPoeVersion } = useStore((state) => ({
    poeVersion: state.poeVersion,
    setPoeVersion: state.setPoeVersion,
  }));

  const checked = poeVersion === 'poe2';
  const handleCheckedChange = useCallback(
    (checked: boolean) => setPoeVersion(checked ? 'poe2' : 'poe1'),
    [setPoeVersion]
  );

  return (
    <div className="flex items-center space-x-2">
      <span className="text-sm font-medium text-gray-700">PoE</span>
      <Switch.Root
        // temporariry disable since I don't have an ability to support poe1 yet
        disabled
        checked={checked}
        onCheckedChange={handleCheckedChange}
        className={`relative inline-flex h-6 w-12 items-center rounded-full transition-colors duration-200 focus:outline-none ${
          checked ? 'bg-gray-600' : 'bg-gray-400'
        }`}
      >
        <Switch.Thumb
          className={`inline-block h-5 w-5 transform rounded-full bg-white shadow transition-transform duration-200 ${
            checked ? 'translate-x-6' : 'translate-x-1'
          }`}
        />
      </Switch.Root>
      <span className="text-sm font-medium text-gray-700">PoE 2</span>
    </div>
  );
};
