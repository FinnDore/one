export const importGetCurrent = async () => {
    const { getCurrent } = await import('@tauri-apps/api/window');
    return getCurrent();
};

export const window = {
    async closeCurrentWindow() {
        return (await importGetCurrent()).close();
    },
    async maximizeCurrentWindow() {
        return (await importGetCurrent()).maximize();
    },
    async minimizeCurrentWindow() {
        return (await importGetCurrent()).minimize();
    },
    async isCurrentWindowFullscreen() {
        return (await importGetCurrent()).isFullscreen();
    },
    async getCurrentWindow() {
        return await importGetCurrent();
    },
};
