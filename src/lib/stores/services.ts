import { writable } from 'svelte/store';
import type { Service } from '../types';
import Database from '@tauri-apps/plugin-sql';

let db: any = null;

async function initDb() {
    if (!db) {
        db = await Database.load('sqlite:tauri-messenger.db');
        // Crear la tabla si no existe
        await db.execute(`
            CREATE TABLE IF NOT EXISTS services (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                url TEXT NOT NULL,
                icon TEXT,
                "order" INTEGER NOT NULL,
                custom_css TEXT,
                custom_js TEXT,
                user_agent TEXT,
                zoom REAL NOT NULL DEFAULT 1.0,
                notifications_enabled INTEGER NOT NULL DEFAULT 1,
                is_active INTEGER NOT NULL DEFAULT 1
            );
        `);
    }
    return db;
}

export const services = writable<Service[]>([]);
export const activeServiceId = writable<string | null>(null);

export async function loadServices() {
    await initDb();
    const result = await db.select<Service[]>('SELECT * FROM services ORDER BY "order"');
    services.set(result);
}

export async function addService(service: Omit<Service, 'id'>) {
    await initDb();
    const id = crypto.randomUUID();
    const order = service.order;
    const icon = service.icon || '';
    const custom_css = service.custom_css || '';
    const custom_js = service.custom_js || '';
    const user_agent = service.user_agent || '';
    
    await db.execute(
        `INSERT INTO services (id, name, url, icon, "order", custom_css, custom_js, user_agent, zoom, notifications_enabled, is_active)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
        [id, service.name, service.url, icon, order, custom_css, custom_js, user_agent, service.zoom, service.notifications_enabled ? 1 : 0, service.is_active ? 1 : 0]
    );
    
    const newService: Service = { ...service, id };
    services.update(s => [...s, newService]);
}

export async function updateService(service: Service) {
    await initDb();
    await db.execute(
        `UPDATE services SET name=?, url=?, icon=?, "order"=?, custom_css=?, custom_js=?, user_agent=?, zoom=?, notifications_enabled=?, is_active=? WHERE id=?`,
        [service.name, service.url, service.icon || '', service.order, service.custom_css || '', service.custom_js || '', service.user_agent || '', service.zoom, service.notifications_enabled ? 1 : 0, service.is_active ? 1 : 0, service.id]
    );
    services.update(s => s.map(svc => svc.id === service.id ? service : svc));
}

export async function deleteService(id: string) {
    await initDb();
    await db.execute('DELETE FROM services WHERE id=?', [id]);
    services.update(s => s.filter(svc => svc.id !== id));
}