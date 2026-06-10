import { Store } from '@tauri-apps/plugin-store';

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