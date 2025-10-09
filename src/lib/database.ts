import { invoke } from '@tauri-apps/api/core';

export interface Account {
  id: string;
  name: string;
  account_type: string;
  balance: number;
  currency: string;
  created_at: string;
  updated_at: string;
  owner: string;  // ADD THIS LINE
}

export interface Category {
  id: string;
  name: string;
  color: string;
  icon: string;
  category_type: 'income' | 'expense';
  created_at: string;
}

export interface Tag {
  id: string;
  name: string;
  color: string;
  icon: string;
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

  static async getTransactionsByMonth(year: number, month: number): Promise<Transaction[]> {
    return await invoke('get_transactions_by_month', { year, month });
  }

  static async getTransactionsByAccount(accountId: string): Promise<Transaction[]> {
    return await invoke('get_transactions_by_account', { accountId });
  }

  static async getTransactionsByCategory(categoryId: string): Promise<Transaction[]> {
    return await invoke('get_transactions_by_category', { categoryId });
  }

  static async getTransactionsByDateRange(startDate: string, endDate: string): Promise<Transaction[]> {
    return await invoke('get_transactions_by_date_range', { startDate, endDate });
  }

  static async deleteTransaction(id: string): Promise<void> {
    await invoke('delete_transaction', { id });
  }

  static async deleteMultipleTransactions(transactionIds: string[]): Promise<number> {
    return await invoke('delete_multiple_transactions', { transactionIds });
  }

  static async bulkUpdateTransactionTags(
    transactionIds: string[],
    tagsToAdd: string[],
    tagsToRemove: string[]
  ): Promise<number> {
    return await invoke('bulk_update_transaction_tags', {
      transactionIds,
      tagsToAdd,
      tagsToRemove
    });
  }

  static async updateTransaction(
    id: string,
    accountId: string,
    categoryId: string,
    amount: number,
    description: string,
    transactionType: 'income' | 'expense',
    date: string
  ): Promise<Transaction> {
    return await invoke('update_transaction', {
      id,
      accountId,
      categoryId,
      amount,
      description,
      transactionType,
      date
    });
  }

  // Category operations
  static async getCategories(): Promise<Category[]> {
    return await invoke('get_categories');
  }

  static async createCategory(
    name: string,
    icon: string,
    categoryType: 'income' | 'expense',
    color: string = '#3B82F6'
  ): Promise<Category> {
    return await invoke('create_category', {
      name,
      icon,
      categoryType,
      color,
    });
  }

  static async updateCategory(
    id: string,
    name: string,
    icon: string,
    categoryType: 'income' | 'expense',
    color: string
  ): Promise<Category> {
    return await invoke('update_category', {
      id,
      name,
      icon,
      categoryType,
      color,
    });
  }

  static async deleteCategory(id: string): Promise<void> {
    await invoke('delete_category', { id });
  }

  // Tag operations
  static async getTags(): Promise<Tag[]> {
    return await invoke('get_tags');
  }

  static async createTag(
    name: string,
    icon: string,
    color: string = '#8B5CF6'
  ): Promise<Tag> {
    return await invoke('create_tag', {
      name,
      icon,
      color,
    });
  }

  static async updateTag(
    id: string,
    name: string,
    icon: string,
    color: string
  ): Promise<Tag> {
    return await invoke('update_tag', {
      id,
      name,
      icon,
      color,
    });
  }

  static async deleteTag(id: string): Promise<void> {
    await invoke('delete_tag', { id });
  }

  // Transaction-Tag relationship operations
  static async addTagsToTransaction(transactionId: string, tagIds: string[]): Promise<void> {
    await invoke('add_tags_to_transaction', {
      transactionId,
      tagIds,
    });
  }

  static async removeTagsFromTransaction(transactionId: string, tagIds: string[]): Promise<void> {
    await invoke('remove_tags_from_transaction', {
      transactionId,
      tagIds,
    });
  }

  static async getTransactionTags(transactionId: string): Promise<Tag[]> {
    return await invoke('get_transaction_tags', { transactionId });
  }

  static async getTransactionsByTag(tagId: string): Promise<Transaction[]> {
    return await invoke('get_transactions_by_tag', { tagId });
  }
}