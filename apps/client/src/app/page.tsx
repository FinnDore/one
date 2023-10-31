'use client';

import { useState } from 'react';
import type { NextPage } from 'next';

import MacTrafficLights from '~/components/traffic-lights';

const Home: NextPage = () => {
    const [added, setAdded] = useState(true);
    return (
        <div className="flex h-screen w-screen flex-col rounded-lg border border-white/25 bg-black">
            <div
                className="h-min w-screen ps-4 pt-4"
                data-tauri-drag-region="true"
            >
                <MacTrafficLights />
            </div>

            <div className="flex h-full flex-1 flex-col items-center justify-center">
                {added ? <AddLight /> : <Light />}
            </div>
        </div>
    );
};

export default Home;

const AddLight = () => {
    return (
        <div className="pointer relative aspect-square h-96">
            <div className="add-light-bg-gradient center-absolute absolute h-full w-full"></div>
            <img
                src="/add-border.svg"
                className="center-absolute absolute h-full  w-full"
            />
            <div className="absolute h-[96%] w-[96%] translate-x-[2%] translate-y-[2%]">
                <Noise />

                <img
                    src="/add-border.svg"
                    className="center-absolute add-light-inner-shadow absolute h-full w-full rounded-md"
                />
                <div className="add-light-bg h-full w-full"></div>
            </div>
            <div className="add-light-bg absolute h-full w-full"></div>
        </div>
    );
};

const Light = () => {
    const [color] = useState('#7527d3');
    return (
        <div className="one border-white/35 relative aspect-square h-64 cursor-pointer overflow-hidden rounded-md border shadow-inner  shadow-black transition-transform hover:scale-[101%]">
            <div
                className="noise absolute h-full w-full"
                style={{
                    background: color,
                }}
            ></div>
            <div
                className="noise absolute h-full w-full opacity-60 blur-lg hue-rotate-[25deg]"
                style={{
                    background: `radial-gradient(transparent 50%, ${color} 100%)`,
                }}
            ></div>

            <Noise />
        </div>
    );
};

const Noise = () => (
    <>
        <img
            src={'/NOISE.png'}
            className="center-absolute absolute h-full w-full rounded-md"
        />
        <img
            src="/noise/2.png"
            className="center-absolute absolute h-full w-full rounded-md"
        />
    </>
);
