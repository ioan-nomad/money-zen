// AccountRepository.ts - EXEMPLU pentru Sonnet
import { invoke } from '@tauri-apps/api/core';
import type { Account } from '../entities/Account';

class AccountRepository {
  async getAll(): Promise<Account[]> {
    try {
      return await invoke('get_accounts');
    } catch (error) {
      console.error('AccountRepository.getAll error:', error);
      throw new Error('Failed to fetch accounts');
    }
  }

  async create(account: Omit<Account, 'id'>): Promise<Account> {
    try {
      return await invoke('create_account', { account });
    } catch (error) {
      console.error('AccountRepository.create error:', error);
      throw new Error('Failed to create account');
    }
  }

  async update(id: string, account: Partial<Account>): Promise<Account> {
    try {
      return await invoke('update_account', { id, account });
    } catch (error) {
      console.error('AccountRepository.update error:', error);
      throw new Error('Failed to update account');
    }
  }

  async delete(id: string): Promise<void> {
    try {
      await invoke('delete_account', { id });
    } catch (error) {
      console.error('AccountRepository.delete error:', error);
      throw new Error('Failed to delete account');
    }
  }
}

export const accountRepository = new AccountRepository();
