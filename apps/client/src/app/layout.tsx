'use client';

import { type ReactElement } from 'react';

import '../styles/globals.css';

const RootLayout = ({ children }: { children: ReactElement }) => {
    return (
        <html>
            <head />
            <body className="h-screen">{children}</body>
        </html>
    );
};

export default RootLayout;
