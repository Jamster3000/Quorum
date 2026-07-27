/**
 * Authentication token management and JWT validation.
 * 
 * Provides utilities for checking login status, managing access tokens,
 * and detecting token expiry with automatic refresh capability. 
 * Integrates with the Tauri Store for persistant (secure) token storage.
 * 
 * Please note as of now, the refresh and access token expiry and refesh
 * do not work. This is a bigger job that is needed to be done with the 
 * server in mind.
 */

import { getAuthStore, getRefreshToken, getUserId } from '$lib/stores/authStore';
import { invoke } from '@tauri-apps/api/core';

let lastCheckTime = 0;
const CHECK_DEBOUNCE_MS = 1000;

/**
 * Check if the user is currently logged in.
 *
 * Retrieves the access token from the Tauri Store and returns true if it exists.
 * Does not validate token expiry or signature—use `isTokenValid()` for that.
 *
 * @returns true if an access token is stored, false otherwise.
 * @throws {Error} If the store operation fails (e.g., I/O error).
 *
 * @example
 * ```typescript
 * if (await isLoggedIn()) {
 *   goto('/home');
 * } else {
 *   goto('/login');
 * }
 * ```
 */
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

/**
 * Retrieve the currently stored access token.
 *
 * Returns null if no token is stored or if the store operation fails.
 * Does not validate expiry; use `isTokenValid()` to ensure the token is still active.
 *
 * @returns The stored access token string, or null if not found.
 *
 * @example
 * ```typescript
 * const token = await getAccessToken();
 * if (token) {
 *   // Use token in API requests
 * }
 * ```
 */
export async function getAccessToken(): Promise<string | null> {
    const store = await getAuthStore(); 
    return (await store.get('access_token')) ?? null;
}

/**
 * Refresh the access token using the stored refresh token.
 *
 * Makes an async request to the Tauri backend's `refresh_token` command.
 * On success, updates both the access token and refresh token in the store
 * (as per JWT rotation strategy). Returns false on any error without throwing.
 *
 * @returns true if the token was successfully refreshed, false otherwise.
 * @throws Does not throw; errors are logged and false is returned.
 *
 * @example
 * ```typescript
 * const success = await refreshAccessToken();
 * if (!success) {
 *   // Token refresh failed; redirect to login
 *   await clearAuthStore();
 *   goto('/login');
 * }
 * ```
 */
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

/**
 * Check if the access token is valid and not about to expire.
 *
 * Decodes the JWT payload, extracts the expiry time, and compares it to the
 * current time with a 5-minute buffer. If the token is expiring soon, this
 * function automatically requests a refresh.
 *
 * Calls are debounced to prevent excessive validation checks; the same token
 * will not be validated more frequently than CHECK_DEBOUNCE_MS.
 *
 * @returns true if the token is valid and not expiring soon, false otherwise.
 *
 * @example
 * ```typescript
 * const valid = await isTokenValid();
 * if (!valid) {
 *   // Token is invalid or will expire soon; user should be logged out
 *   await clearAuthStore();
 *   goto('/login');
 * }
 * ```
 */
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