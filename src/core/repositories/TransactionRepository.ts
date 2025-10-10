// TransactionRepository.ts - CRUD pentru Transactions
import { invoke } from '@tauri-apps/api/core';
import type { Transaction } from '../entities/Transaction';

class TransactionRepository {
  async getAll(): Promise<Transaction[]> {
    try {
      return await invoke('get_transactions');
    } catch (error) {
      console.error('TransactionRepository.getAll error:', error);
      throw new Error('Failed to fetch transactions');
    }
  }

  async getByAccount(accountId: string): Promise<Transaction[]> {
    try {
      return await invoke('get_transactions_by_account', { accountId });
    } catch (error) {
      console.error('TransactionRepository.getByAccount error:', error);
      throw new Error('Failed to fetch transactions by account');
    }
  }

  async getByCategory(categoryId: string): Promise<Transaction[]> {
    try {
      return await invoke('get_transactions_by_category', { categoryId });
    } catch (error) {
      console.error('TransactionRepository.getByCategory error:', error);
      throw new Error('Failed to fetch transactions by category');
    }
  }

  async getByDateRange(startDate: string, endDate: string): Promise<Transaction[]> {
    try {
      return await invoke('get_transactions_by_date_range', { startDate, endDate });
    } catch (error) {
      console.error('TransactionRepository.getByDateRange error:', error);
      throw new Error('Failed to fetch transactions by date range');
    }
  }

  async create(transaction: Omit<Transaction, 'id' | 'created_at' | 'updated_at'>): Promise<Transaction> {
    try {
      return await invoke('create_transaction', {
        accountId: transaction.account_id,
        categoryId: transaction.category_id,
        amount: transaction.amount,
        description: transaction.description,
        transactionType: transaction.transaction_type,
        date: transaction.date,
        tagIds: transaction.tags || []
      });
    } catch (error) {
      console.error('TransactionRepository.create error:', error);
      throw new Error('Failed to create transaction');
    }
  }

  async update(id: string, transaction: Partial<Transaction>): Promise<Transaction> {
    try {
      return await invoke('update_transaction', {
        id,
        accountId: transaction.account_id,
        categoryId: transaction.category_id,
        amount: transaction.amount,
        description: transaction.description,
        transactionType: transaction.transaction_type,
        date: transaction.date
      });
    } catch (error) {
      console.error('TransactionRepository.update error:', error);
      throw new Error('Failed to update transaction');
    }
  }

  async delete(id: string): Promise<void> {
    try {
      await invoke('delete_transaction', { id });
    } catch (error) {
      console.error('TransactionRepository.delete error:', error);
      throw new Error('Failed to delete transaction');
    }
  }

  async deleteMultiple(transactionIds: string[]): Promise<number> {
    try {
      return await invoke('delete_multiple_transactions', { transactionIds });
    } catch (error) {
      console.error('TransactionRepository.deleteMultiple error:', error);
      throw new Error('Failed to delete transactions');
    }
  }

  async bulkUpdateTags(transactionIds: string[], tagsToAdd?: string[], tagsToRemove?: string[]): Promise<number> {
    try {
      return await invoke('bulk_update_transaction_tags', {
        transactionIds,
        tagsToAdd,
        tagsToRemove
      });
    } catch (error) {
      console.error('TransactionRepository.bulkUpdateTags error:', error);
      throw new Error('Failed to bulk update tags');
    }
  }
}

export const transactionRepository = new TransactionRepository();