/**
 * Authentication Storage Management
 * 
 * Provides functions to get and store refresh tokens and access tokens
 * in Tauri Store.
 */

import { Store } from '@tauri-apps/plugin-store';

let storeInstance: Store | null = null;

/**
 * Get store
 * 
 * Gets the instance of the store.
 * If the instance of the store has already been created in the user session,
 * (stored in storeInstance) it will return that instance rather than reading and loading the file in
 * to memory a second time.
 * 
 * @returns Promise<Store> - The instance of the store.
 * 
 * @example
 * ```typescript
 * const store = await getAuthStore();
 * ```
 */
export async function getAuthStore(): Promise<Store> {
    if (!storeInstance) {
        storeInstance = await Store.load('.auth.dat');
    }
    return storeInstance;
}

/**
 * Get the access token
 * 
 * Gets the Tauri store (create or get current instance)
 * and retrieves the access token from the store.
 * 
 * @returns Promise<string | null> - The access token or null if not found.
 * @throws {Error} If the store operation fails (e.g., I/O error).
 * 
 * @example
 * ```typescript
 * const accessToken = await getAccessToken();
 * ```
 */
export async function getAccessToken(): Promise<string | null> {
    try {
        const store = await getAuthStore();
        return (await store.get('access_token')) ?? null;
    } catch (error) {
        console.error('Failed to get access token:', error);
        return null;
    }
}

/**
 * Get the refresh token
 * 
 * Gets the Tauri store (create or get current instance)
 * and retrieves the refresh token from the store.
 * 
 * @returns Promise<string | null> - The refresh token or null if not found.
 * @throws {Error} If the store operation fails (e.g., I/O error).
 * 
 * @example
 * ```Typescript
 * const refreshToken = await getRefreshToken();
 * ```
 */
export async function getRefreshToken(): Promise<string | null> {
    try {
        const store = await getAuthStore();
        return (await store.get('refresh_token')) ?? null;
    } catch (error) {
        console.error('Failed to get refresh token:', error);
        return null;
    }
}

/**
 * Get the user ID
 * 
 * Gets the Tauri store (create or get current instance)
 * and retrieves the user ID from the store.
 * 
 * @returns Promise<string | null> - The user ID or null if not found.
 * @throws {Error} If the store operation fails (e.g., I/O error).
 * 
 * @example
 * ```typescript
 * const userId = await getUserId();
 * ```
 */
export async function getUserId(): Promise<string | null> {
    try {
        const store = await getAuthStore();
        return (await store.get('user_id')) ?? null;
    } catch (error) {
        console.error('Failed to get user ID:', error);
        return null;
    }
}

/**
 * Get the username
 * 
 * Gets the Tauri store (create or get current instance)
 * and retrieves the username from the store.
 * 
 * @returns Promise<string | null> - The username or null if not found.
 * @throws {Error} If the store operation fails (e.g., I/O error).
 * 
 * @example
 * ```Typescript
 * const username = await getUsername();
 * ```
 */
export async function getUsername(): Promise<string | null> {
    try {
        const store = await getAuthStore();
        return (await store.get('username')) ?? null
    } catch (error) {
        console.error('Failed to get username:', error);
        return null;
    }
}

/**
 * Sets the authentication information in the Tauri store.
 * 
 * @param accessToken - The access token to store.
 * @param refreshToken - The refresh token to store.
 * @param userId - The user ID to store.
 * @param username - The username to store.
 * 
 * @returns Promise<void> - Resolves when the values are successfully stored.
 * @throws {Error} If the store operation fails (e.g., I/O error).
 * 
 * @example 
 * ```typescript
 * await setAuthStoreValues('accessToken123', 'refreshToken456','userId789', 'usernameExample');
 * ```
 */
export async function setAuthStoreValues(
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

/**
 * Clear Tauri store
 * 
 * Clears the Tauri authentication store by removing all stored values.
 * 
 * @returns Promise<void> - Resolves when the store is successfully cleared.
 * @throws {Error} If the store operation fails (e.g., I/O error).
 * 
 * @example
 * ```typescript
 * await clearAuthStore();
 * ```
 */
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

/**
 * Reinitialize the authentication store
 * 
 * Reinitializes the authentication store by setting the store instance to null
 * and then loading the store from the file again. 
 * 
 * Loading the store in on most low-medium to high devices will cause 
 * an unnoticable delay. This is ideal for when the store needs to be cleared from memory and reloaded again.
 * 
 * @returns Promise<void> - Resolves when the store is successfully reinitialized.
 * 
 * @example
 * ```typescript
 * await reinitializeAuthStore();
 * ```
 */
export async function reinitializeAuthStore(): Promise<void> {
    storeInstance = null;
    storeInstance = await Store.load('.auth.dat');
}
