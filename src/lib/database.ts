import { invoke } from '@tauri-apps/api/core';

export interface Account {
  id: string;
  name: string;
  account_type: string;
  balance: number;
  currency: string;
  created_at: string;
  updated_at: string;
}

export interface Category {
  id: string;
  name: string;
  color: string;
  icon: string;
  category_type: 'income' | 'expense';
  created_at: string;
}

export interface Transaction {
  id: string;
  account_id: string;
  category_id: string;
  amount: number;
  description: string;
  transaction_type: 'income' | 'expense';
  date: string;
  created_at: string;
  updated_at: string;
}

export class Database {
  static async initDatabase(): Promise<void> {
    await invoke('init_database');
  }

  // Account operations
  static async createAccount(
    name: string,
    accountType: string,
    currency: string = 'RON'
  ): Promise<Account> {
    return await invoke('create_account', {
      name,
      accountType,
      currency,
    });
  }

  static async getAccounts(): Promise<Account[]> {
    return await invoke('get_accounts');
  }

  static async updateAccount(
    id: string,
    name: string,
    accountType: string,
    currency: string = 'RON'
  ): Promise<Account> {
    return await invoke('update_account', {
      id,
      name,
      accountType,
      currency,
    });
  }

  static async deleteAccount(id: string): Promise<void> {
    await invoke('delete_account', { id });
  }

  static async backupDatabase(): Promise<string> {
    return await invoke<string>('backup_database');
  }

  static async restoreDatabase(backupPath: string): Promise<string> {
    return await invoke<string>('restore_database', { backupPath });
  }

  // Transaction operations
  static async createTransaction(
    accountId: string,
    categoryId: string,
    amount: number,
    description: string,
    transactionType: 'income' | 'expense',
    date: string = new Date().toISOString()
  ): Promise<Transaction> {
    return await invoke('create_transaction', {
      accountId,
      categoryId,
      amount,
      description,
      transactionType,
      date,
    });
  }

  static async getTransactions(): Promise<Transaction[]> {
    return await invoke('get_transactions');
  }

  // Category operations
  static async getCategories(): Promise<Category[]> {
    return await invoke('get_categories');
  }
}