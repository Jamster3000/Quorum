import { Store } from '@tauri-apps/plugin-store';
import { invoke } from '@tauri-apps/api/core';

export async function isLoggedIn() {
    try {
        const store = await Store.load('.auth.dat');
        const accessToken = await store.get('access_token');
        return !!accessToken; 
    } catch (e) {
        console.error('Failed to check login status:', e);
        return false;
    }
}

export async function getAccessToken(): Promise<string | null> {
    const store = await Store.load('.auth.dat');
    return await store.get('access_token');
}

export async function refreshAccessToken(): Promise<boolean> {
    const store = await Store.load('.auth.dat');
    const refreshToken = await store.get('refresh_token');

    if (!refreshToken) {
        console.error('No refresh token available');
        return false;
    }

    try {
        const result = await invoke < {
            success: boolean;
            message: string;
            access_token?: string;
            refresh_token?: string;
        } > ('refresh_token', { refreshToken });

        if (result.success && result.access_token) {
            await store.set('access_token', result.access_token);
            if (result.refresh_token) {
                await store.set('refresh_token', result.refresh_token);
            }
            await store.save();
            return true;
        } else {
            console.error('Failed to refresh token:', result.message);
            return false;
        }
    } catch (e) {
        console.error('Error refreshing token:', e);
        return false;
    }
}

export async function isTokenValid(): Promise<boolean> {
    console.log("Checking token validity...");
    const store = await Store.load('.auth.dat');
    const accessToken = await store.get('access_token');

    if (!accessToken) {
        return false;
    }

    try {
        const payload = JSON.parse(atob(accessToken.split('.')[1]));
        const expiresAt = payload.exp * 1000;
        const now = Date.now();
        const buffer = 5 * 60 * 1000; // 5-minute buffer (refresh 5 mins before expiry)

        console.log(`Token expires at: ${new Date(expiresAt).toISOString()}`);
        console.log(`Current time: ${new Date(now).toISOString()}`);
        console.log(`Time until expiry: ${(expiresAt - now) / 1000} seconds`);

        if (expiresAt - now < buffer) {
            console.log("Token is about to expire, refreshing...");
            return await refreshAccessToken();
        }
        return true;
    } catch (e) {
        console.error('Error validating token:', e);
        return false;
    }
}