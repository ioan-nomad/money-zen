import { notificationStore } from './notificationStore';
// accountStore.ts - EXEMPLU pentru Sonnet
import { writable } from 'svelte/store';
import { accountRepository } from '../../core/repositories/AccountRepository';
import type { Account } from '../../core/entities/Account';

function createAccountStore() {
  const { subscribe, set, update } = writable<Account[]>([]);
  
  return {
    subscribe,
    
    async load() {
      try {
        const accounts = await accountRepository.getAll();
        set(accounts);
      } catch (error) {
        console.error('Failed to load accounts:', error);
        notificationStore.error('Failed to load accounts');
      }
    },
    
    async create(account: Omit<Account, 'id'>) {
      try {
        const newAccount = await accountRepository.create(account);
        update(accounts => [...accounts, newAccount]);
        notificationStore.success('Account created');
        return newAccount;
      } catch (error) {
        console.error('Failed to create account:', error);
        notificationStore.error('Failed to create account');
        throw error;
      }
    },
    
    async remove(id: string) {
      try {
        await accountRepository.delete(id);
        update(accounts => accounts.filter(a => a.id !== id));
        notificationStore.success('Account deleted');
      } catch (error) {
        console.error('Failed to delete account:', error);
        notificationStore.error('Failed to delete account');
        throw error;
      }
    }
  };
}

export const accountStore = createAccountStore();
