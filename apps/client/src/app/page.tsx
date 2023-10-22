'use client';

import { useState } from 'react';
import type { NextPage } from 'next';

import MacTrafficLights from '~/components/traffic-lights';

const Home: NextPage = () => {
    const [added, setAdded] = useState(false);
    return (
        <div className="flex h-screen w-screen flex-col rounded-lg border border-white/25 bg-black">
            <div
                className="h-min w-screen ps-4 pt-4"
                data-tauri-drag-region="true"
            >
                <MacTrafficLights />
            </div>

            <div
                className="flex h-full flex-1 flex-col items-center justify-center"
                onClick={() => setAdded(val => !val)}
            >
                {added ? <AddLight /> : <Light />}
            </div>
        </div>
    );
};

export default Home;

const AddLight = () => {
    return (
        <div className="border-white/35 pointer relative flex aspect-square h-64 cursor-pointer overflow-hidden rounded-md border border-dashed bg-white/5 transition-transform hover:scale-[101%]">
            <div className="m-auto flex select-none flex-col gap-2 text-center leading-none text-white opacity-80">
                <div className="text-3xl">+</div>
                <div className="text-1xl">Add light</div>
            </div>
        </div>
    );
};

const Light = () => {
    const [color] = useState('#7527d3');
    return (
        <div
            className="one border-white/35 relative aspect-square h-64 cursor-pointer overflow-hidden rounded-md border transition-transform hover:scale-[101%]"
            style={{ backgroundColor: color }}
        >
            <Noise />
        </div>
    );
};

const Noise = (props: { className?: string }) => (
    <div
        className={`noise absolute z-10 h-full w-full ${props.className}`}
    ></div>
);
