import { getAuthStore, getRefreshToken, getUserId } from '$lib/stores/authStore';
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
    const userId = await getUserId();

    if (!refreshToken || !userId) {
        console.error('✗ Missing refresh token or user ID');
        return false;
    }

    console.log('→ Attempting to refresh access token...');

    try {
        const result = await invoke<{
            success: boolean;
            message: string;
            access_token?: string;
            refresh_token?: string;
        }>('refresh_token', { refreshToken, userId });

        if (result.success && result.access_token) {
            const store = await getAuthStore();
            await store.set('access_token', result.access_token);
            if (result.refresh_token) {
                await store.set('refresh_token', result.refresh_token);
            }
            await store.save();
            console.log('✓ Access token refreshed successfully');
            return true;
        } else {
            console.error('✗ Refresh failed:', result.message);
            return false;
        }
    } catch (e) {
        console.error('✗ Error refreshing token:', e);
        return false;
    }
}

export async function isTokenValid(): Promise<boolean> {
    const store = await getAuthStore();
    const accessToken = await store.get<string>('access_token');

    if (!accessToken) {
        console.log('✗ No access token found');
        return false;
    }

    try {
        const base64Url = accessToken.split('.')[1];
        const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
        const payload = JSON.parse(atob(base64)) as { exp: number };
        const expiresAt = payload.exp * 1000;
        const now = Date.now();
        const buffer = 5 * 60 * 1000;

        console.log(`Token expires at: ${new Date(expiresAt).toISOString()}, now: ${new Date(now).toISOString()}`);

        if (expiresAt - now < buffer) {
            console.log('→ Token expiring soon, refreshing...');
            return await refreshAccessToken();
        }
        console.log('✓ Token still valid');
        return true;
    } catch (e) {
        console.error('✗ Error decoding token:', e);
        return false;
    }
}