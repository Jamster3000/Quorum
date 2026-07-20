import { Store } from '@tauri-apps/plugin-store';

let storeInstance: Store | null = null;

export async function getAuthStore(): Promise<Store> {
    if (!storeInstance) {
        storeInstance = await Store.load('.auth.dat');
    }
    return storeInstance;
}

export async function getAccessToken(): Promise<string | null> {
    try {
        const store = await getAuthStore();
        return (await store.get('access_token')) ?? null;
    } catch (error) {
        console.error('Failed to get access token:', error);
        return null;
    }
}

export async function getRefreshToken(): Promise<string | null> {
    try {
        const store = await getAuthStore();
        return (await store.get('refresh_token')) ?? null;
    } catch (error) {
        console.error('Failed to get refresh token:', error);
        return null;
    }
}

export async function getUserId(): Promise<string | null> {
    try {
        const store = await getAuthStore();
        return (await store.get('user_id')) ?? null;
    } catch (error) {
        console.error('Failed to get user ID:', error);
        return null;
    }
}

export async function getUsername(): Promise<string | null> {
    try {
        const store = await getAuthStore();
        return (await store.get('username')) ?? null
    } catch (error) {
        console.error('Failed to get username:', error);
        return null;
    }
}

export async function setTokens(
    accessToken: string,
    refreshToken: string,
    userId: string,
    username: string
): Promise<void> {
    try {
        const store = await getAuthStore();
        await store.set('access_token', accessToken);
        await store.set('refresh_token', refreshToken);
        await store.set('user_id', userId);
        await store.set('username', username);
        await store.save();
        console.log('Tokens saved to store');
    } catch (error) {
        console.error('Failed to set tokens:', error);
        throw error;
    }
}

export async function clearAuthStore(): Promise<void> {
    try {
        const store = await getAuthStore();
        await store.clear();
        await store.save();
    } catch (error) {
        console.error('Failed to clear auth store:', error);
        throw error;
    }
}

export async function reinitializeAuthStore(): Promise<void> {
    storeInstance = null;
    storeInstance = await Store.load('.auth.dat');
}
