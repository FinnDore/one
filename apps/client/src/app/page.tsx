/* eslint-disable @next/next/no-img-element */
'use client';

import { useState } from 'react';
import type { NextPage } from 'next';
import clsx from 'clsx';

import { MacTrafficLights } from '~/components/traffic-lights';
import { commands } from '~/lazy-tauri-api/get-current';

const Home: NextPage = () => {
    const [added, setAdded] = useState(true);
    const [color, setColor] = useState({
        hex: '#5F00A9',
        rgb: { r: 95, g: 0, b: 169 },
    });

    return (
        <div className="flex h-screen w-screen flex-col rounded-lg border border-white/25 bg-black">
            <div className="full h-min ps-4 pt-4" data-tauri-drag-region="true">
                <MacTrafficLights />
            </div>
            <input
                type="color"
                className="absolute"
                onChange={e => {
                    setColor(() => ({
                        hex: e.target.value,
                        rgb: hexToRgbString(e.target.value),
                    }));
                    void commands.set_color(e.target.value);
                }}
            />

            <div
                className="relative flex h-full flex-1 flex-col items-center justify-center overflow-x-clip"
                onClick={() => setAdded(v => !v)}
            >
                {added ? (
                    <Light name="Living room" color={color} />
                ) : (
                    <AddLight />
                )}
            </div>
        </div>
    );
};

export default Home;

const AddLight = () => {
    return (
        <div className="group relative aspect-square h-96 cursor-pointer select-none transition-all duration-300 hover:scale-105">
            <Dots purple={false} />
            <div className="add-light-text absolute w-full -translate-y-[125%] text-center text-3xl font-bold text-white">
                Add Light
            </div>
            <div
                className="add-light-bg-gradient absolute h-full w-full blur-[30px]"
                style={{
                    background:
                        `conic-gradient(from 44deg at 20% 45%,` +
                        'rgba(255, 255, 255, 0) 106deg,' +
                        'rgba(161, 161, 161, 0.38) 194deg,' +
                        'rgba(255, 255, 255, 0.3) 333.7499928474426deg)',
                }}
            ></div>
            <div
                className="add-light-bg-gradient pointer-events-none absolute h-[200%] w-[200%] -translate-x-1/4 -translate-y-1/4 opacity-50 blur-[120px]"
                style={{
                    background:
                        `conic-gradient(from 44deg at 50% 55%,` +
                        'rgba(255, 255, 255, 0) 106deg,' +
                        'rgba(161, 161, 161, 0.38) 194deg,' +
                        'rgba(255, 255, 255, 0.3) 333.7499928474426deg)',
                }}
            ></div>
            <img
                alt=""
                src="/add-border.svg"
                className=" absolute h-full  w-full"
            />
            <div className="absolute h-[97%] w-[97%] translate-x-[1.5%] translate-y-[1.5%]">
                <img
                    alt=""
                    src={'/NOISE.png'}
                    className="absolute h-full w-full rounded-md"
                />
                <img
                    alt=""
                    src="/noise/2.png"
                    className="absolute h-full w-full rounded-md"
                />
                <img
                    alt=""
                    src="/add-border.svg"
                    className="add-light-inner-shadow absolute h-full w-full rounded-md"
                />
                <div
                    className="absolute h-full w-full blur-md"
                    style={{
                        background:
                            'conic-gradient(from 43deg at 55.2% 53.52%,' +
                            'rgba(223, 223, 223, 0) 110.62499642372131deg,' +
                            'rgba(220, 217, 225, 0.05) 183.23912143707275deg,' +
                            'rgba(250, 250, 250, 0.08) 206.25000715255737deg,' +
                            'rgba(223, 223, 223, 0.06) 308.52407455444336deg,' +
                            'rgba(219, 216, 225, 0.06) 333.7499928474426deg)',
                    }}
                ></div>

                <div className="absolute grid h-full w-full place-content-center text-7xl transition-all group-hover:scale-110">
                    <span
                        className="text-gradient text-clip"
                        style={{
                            backgroundImage:
                                'radial-gradient(64.86% 92.48% at 32.56% 36.72%,' +
                                '#fff 0%,' +
                                'rgba(255, 255, 255, 0.42) 22.87%,' +
                                'rgba(255, 255, 255, 0) 100%)',
                        }}
                    >
                        +
                    </span>
                </div>
            </div>
        </div>
    );
};

const Dots = (props: { purple?: boolean; hue?: number }) => (
    <div
        className="center-absolute transform-all pointer-events-none absolute h-[150%] w-screen"
        style={{
            filter: `hue-rotate(${props.hue ?? 0}deg)`,
        }}
    >
        <div
            className={clsx(
                'absolute h-full w-[200%] -translate-x-1/4 bg-repeat transition-all duration-300 group-hover:scale-[.85]',
                {
                    dots: !props.purple,
                    'dots-purple': props.purple,
                },
            )}
        ></div>
        <div
            className="absolute h-full w-full blur-xl"
            style={{
                background:
                    'linear-gradient(transparent black, 0 50%, black 100%)',
            }}
        ></div>
        <div
            className="absolute h-full w-full blur-xl"
            style={{
                background:
                    'linear-gradient(90deg, transparent 0, black 50%, transparent 100%)',
            }}
        ></div>
    </div>
);

const Light = (props: {
    name: string;
    color: {
        hex: string;
        rgb: {
            r: number;
            g: number;
            b: number;
        };
    };
}) => {
    return (
        <div className="group relative aspect-square h-96 cursor-pointer select-none transition-all duration-300 hover:scale-105">
            <div className="absolute bottom-0 z-10 translate-y-[125%] text-xl text-white/70">
                {props.name}
            </div>
            <Dots
                hue={props.color.rgb.r * props.color.rgb.g * props.color.rgb.b}
            />

            <div
                className="add-light-bg-gradient absolute h-full w-full overflow-hidden rounded-md"
                style={{
                    background:
                        'linear-gradient(135deg, #FFF -19.54%, rgba(255, 255, 255, 0.00) 116.3%)',
                }}
            >
                <div className="h-full w-full bg-white/30"></div>
            </div>
            <div
                className="add-light-bg-gradient pointer-events-none absolute h-[200%] w-[200%] -translate-x-1/4 -translate-y-1/4 opacity-50 blur-[120px]"
                style={{
                    background:
                        'conic-gradient(from 90deg at 50% 50%,' +
                        `rgba(${combineColor(
                            props.color.rgb.r,
                            48,
                        )}, 0, ${combineColor(
                            props.color.rgb.b,
                            86,
                        )}, 0.00) 110.62499642372131deg,` +
                        `rgba(${props.color.rgb.r + 143}, 0, ${combineColor(
                            props.color.rgb.b,
                            86,
                        )}, 0.38) 206.25000715255737deg,` +
                        `rgba(${combineColor(
                            props.color.rgb.r,
                            -18,
                        )} , 0, ${combineColor(
                            props.color.rgb.b,
                            34,
                        )}, 0.30) 333.7499928474426deg)`,
                }}
            ></div>
            <img
                alt=""
                src="/added-border.svg"
                className=" absolute h-full  w-full"
            />
            <div className="absolute h-[97%] w-[97%] translate-x-[1.5%] translate-y-[1.5%]">
                <div
                    className="absolute h-full w-full rounded-md opacity-95"
                    style={{
                        background: props.color.hex,
                    }}
                ></div>
                <div
                    className="absolute h-full w-full rounded-md blur-md"
                    style={{
                        background:
                            'conic-gradient(from 180deg at 50% 50%,' +
                            `rgba(${combineColor(
                                props.color.rgb.r,
                                48,
                            )}, 0, ${combineColor(
                                props.color.rgb.b,
                                86,
                            )}, 0.00) 110.62499642372131deg,` +
                            `rgba(${props.color.rgb.r + 143}, 0, ${combineColor(
                                props.color.rgb.b,
                                86,
                            )}, 0.38) 206.25000715255737deg,` +
                            `rgba(${combineColor(
                                props.color.rgb.r,
                                -18,
                            )} , 0, ${combineColor(
                                props.color.rgb.b,
                                34,
                            )}, 0.76) 333.7499928474426deg)`,
                    }}
                ></div>
                <img
                    alt=""
                    src={'/NOISE.png'}
                    className="absolute h-full w-full rounded-md"
                />
                <img
                    alt=""
                    src="/noise/2.png"
                    className="absolute h-full w-full rounded-md"
                />
                <img
                    alt=""
                    src="/noise/3.png"
                    className="absolute h-full w-full rounded-md"
                />
                <img
                    alt=""
                    src="/added-border.svg"
                    className="add-light-inner-shadow absolute h-full w-full rounded-md"
                />
            </div>
        </div>
    );
};

const combineColor = (color: number, color2: number) => {
    const sum = color + color2;
    if (sum > 255) {
        return 255;
    } else if (sum < 0) {
        return 0;
    } else {
        return sum;
    }
};

const hexToRgbString = (hex: string) => {
    const bigint = parseInt(hex.replace('#', ''), 16);
    const r = (bigint >> 16) & 255;
    const g = (bigint >> 8) & 255;
    const b = bigint & 255;

    return { r, g, b };
};
