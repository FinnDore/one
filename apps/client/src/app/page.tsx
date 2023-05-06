'use client';

import type { NextPage } from 'next';

import MacTrafficLights from '~/components/traffic-lights';

const Home: NextPage = () => {
    return (
        // tauri drag region
        // https://tauri.studio/en/docs/api/js/tauri/drag-region/

        <div className="flex h-screen w-screen rounded-lg border border-white/25 bg-black">
            <div className="h-min w-screen ps-2 pt-2" data-tauri-drag-region>
                <MacTrafficLights />
            </div>
        </div>
    );
};

export default Home;
