import { getAuthStore, getRefreshToken } from '$lib/stores/authStore';
import { invoke } from '@tauri-apps/api/core';

let lastCheckTime = 0;
const CHECK_DEBOUNCE_MS = 1000;

export async function isLoggedIn() {
    try {
        const store = await getAuthStore();
        const accessToken = await store.get('access_token');
        return !!accessToken;
    } catch (e) {
        console.error('Failed to check login status:', e);
        return false;
    }
}

export async function getAccessToken(): Promise<string | null> {
    const store = await getAuthStore(); 
    return (await store.get('access_token')) ?? null;
}

export async function refreshAccessToken(): Promise<boolean> {
    const refreshToken = await getRefreshToken(); 

    if (!refreshToken) {
        console.error('No refresh token available');
        return false;
    }

    try {
        const result = await invoke<{
            success: boolean;
            message: string;
            access_token?: string;
            refresh_token?: string;
        }>('refresh_token', { refreshToken });

        if (result.success && result.access_token) {
            const store = await getAuthStore();
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
    const now = Date.now();

    if (now - lastCheckTime < CHECK_DEBOUNCE_MS) {
        console.debug('Token check skipped (debounced)');
        return true;
    }
    lastCheckTime = now;

    const store = await getAuthStore(); 
    const accessToken = await store.get('access_token');

    if (!accessToken) {
        return false;
    }

    try {
        const payload = JSON.parse(atob(accessToken.split('.')[1])) as { exp: number };
        const expiresAt = payload.exp * 1000;
        const now = Date.now();
        const buffer = 5 * 60 * 1000;

        console.debug(`Token expires at: ${new Date(expiresAt).toISOString()}`);
        console.debug(`Current time: ${new Date(now).toISOString()}`);
        console.debug(`Time until expiry: ${(expiresAt - now) / 1000} seconds`);

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