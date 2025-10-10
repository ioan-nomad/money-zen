import { writable } from 'svelte/store';

export interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number;
}

function createNotificationStore() {
  const { subscribe, update } = writable<Notification[]>([]);
  
  const addNotification = (
    type: Notification['type'],
    message: string,
    duration = 5000
  ) => {
    const id = Date.now().toString();
    const notification: Notification = { id, type, message, duration };
    
    update(n => [...n, notification]);
    
    if (duration > 0) {
      setTimeout(() => {
        update(n => n.filter(notif => notif.id !== id));
      }, duration);
    }
    
    return id;
  };
  
  return {
    subscribe,
    success: (msg: string, duration?: number) => 
      addNotification('success', msg, duration),
    error: (msg: string, duration?: number) => 
      addNotification('error', msg, duration || 10000),
    warning: (msg: string, duration?: number) => 
      addNotification('warning', msg, duration),
    info: (msg: string, duration?: number) => 
      addNotification('info', msg, duration),
    remove: (id: string) => 
      update(n => n.filter(notif => notif.id !== id))
  };
}

export const notificationStore = createNotificationStore();
