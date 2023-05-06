'use client';

/* eslint-disable @typescript-eslint/no-misused-promises */
/* eslint-disable @next/next/no-img-element */
// Literally just stolen from https://github.com/spacedriveapp/spacedrive/blob/main/interface/components/TrafficLights.tsx#L36
import { useMemo, type HTMLAttributes } from 'react';
import clsx from 'clsx';

import { useFocusState } from '~/hooks/use-focus-state';
import { window } from '~/lazy-tauri-api/get-current';

export default function MacTrafficLights(props: { className?: string }) {
    const [focused] = useFocusState();

    return (
        <div
            data-tauri-drag-region
            className={clsx(
                'group flex flex-row space-x-[7.5px]',
                props.className,
            )}
        >
            <TrafficLight
                type="close"
                onClick={async () => await window.closeCurrentWindow()}
                colorful={focused ?? false}
            />
            <TrafficLight
                type="minimize"
                onClick={async () => await window.minimizeCurrentWindow()}
                colorful={focused ?? false}
            />
            <TrafficLight
                type="fullscreen"
                onClick={async () => {
                    const currentWindow = await window.getCurrentWindow();
                    void currentWindow.setFullscreen(
                        !(await currentWindow.isFullscreen()),
                    );
                }}
                colorful={focused ?? false}
            />
        </div>
    );
}

interface TrafficLightProps {
    type: 'close' | 'minimize' | 'fullscreen';
    colorful: boolean;
    onClick?: HTMLAttributes<HTMLDivElement>['onClick'];
}

function TrafficLight(props: TrafficLightProps) {
    const { onClick = () => undefined, colorful = false, type } = props;
    const iconPath = useMemo(() => {
        switch (type) {
            case 'close':
                return 'macos_close.svg';

            case 'minimize':
                return 'macos_minimize.svg';
            case 'fullscreen':
                return 'macos_fullscreen.svg';
        }
    }, [type]);

    return (
        <div
            onClick={onClick}
            className={clsx(
                'box-content flex h-[12px] w-[12px] items-center justify-center rounded-full border-[0.5px] border-transparent bg-[#CDCED0] dark:bg-[#2B2C2F]',
                {
                    'border-red-900 !bg-[#EC6A5E] active:hover:!bg-red-700 dark:active:hover:!bg-red-300':
                        type === 'close' && colorful,
                    'group-hover:!bg-[#EC6A5E] ': type === 'close',
                    'border-yellow-900 !bg-[#F4BE4F]  active:hover:!bg-yellow-600 dark:active:hover:!bg-yellow-200':
                        type === 'minimize' && colorful,
                    'group-hover:!bg-[#F4BE4F]': type === 'minimize',
                    'border-green-900 !bg-[#61C253]  active:hover:!bg-green-700 dark:active:hover:!bg-green-300':
                        type === 'fullscreen' && colorful,
                    ' group-hover:!bg-[#61C253] ': type === 'fullscreen',
                },
            )}
        >
            <img
                src={`/${iconPath}`}
                alt={`${type} icon`}
                className="pointer-events-none opacity-0 group-hover:opacity-100 group-active:opacity-100"
            />
        </div>
    );
}
