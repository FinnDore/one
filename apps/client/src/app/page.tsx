'use client';

import type { NextPage } from 'next';
import { clsx } from 'clsx';

import MacTrafficLights from '~/components/traffic-lights';

const lights = Array(10).fill(0);

const Home: NextPage = () => {
    console.log(lights);
    return (
        <div className="flex h-screen w-screen rounded-lg border border-white/25 bg-black text-white">
            <div className="w-screen ps-2 pt-2" data-tauri-drag-region>
                <MacTrafficLights />
                <div className="mx-auto mt-10 flex w-[80ch] flex-wrap gap-10">
                    {lights.map((_, i) => (
                        <div key={i} className="my-2">
                            <Light className="" />
                            <div className="mt-1">Light {i}</div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};

export default Home;

const Light = (props: { className?: string }) => {
    return (
        <div
            className={clsx(
                'border-white/55 relative aspect-square w-40 overflow-hidden rounded-md border',
                props.className,
            )}
        >
            <div className="absolute aspect-square h-full  bg-rose-600"></div>
            {/* <div className="aspect-square h-full rounded-md bg-blue-950"></div> */}
        </div>
    );
};
