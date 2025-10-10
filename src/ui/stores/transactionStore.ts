import { notificationStore } from './notificationStore';
// transactionStore.ts - State management pentru Transactions
import { writable } from 'svelte/store';
import { transactionRepository } from '../../core/repositories/TransactionRepository';
import type { Transaction } from '../../core/entities/Transaction';

function createTransactionStore() {
  const { subscribe, set, update } = writable<Transaction[]>([]);

  return {
    subscribe,

    async load() {
      try {
        const transactions = await transactionRepository.getAll();
        set(transactions);
      } catch (error) {
        console.error('Failed to load transactions:', error);
        notificationStore.error('Failed to load transactions');
      }
    },

    async loadByAccount(accountId: string) {
      try {
        const transactions = await transactionRepository.getByAccount(accountId);
        set(transactions);
      } catch (error) {
        console.error('Failed to load transactions by account:', error);
        notificationStore.error('Failed to load transactions');
      }
    },

    async loadByCategory(categoryId: string) {
      try {
        const transactions = await transactionRepository.getByCategory(categoryId);
        set(transactions);
      } catch (error) {
        console.error('Failed to load transactions by category:', error);
        notificationStore.error('Failed to load transactions');
      }
    },

    async loadByDateRange(startDate: string, endDate: string) {
      try {
        const transactions = await transactionRepository.getByDateRange(startDate, endDate);
        set(transactions);
      } catch (error) {
        console.error('Failed to load transactions by date range:', error);
        notificationStore.error('Failed to load transactions');
      }
    },

    async create(transaction: Omit<Transaction, 'id' | 'created_at' | 'updated_at'>) {
      try {
        const newTransaction = await transactionRepository.create(transaction);
        update(transactions => [newTransaction, ...transactions]);
        notificationStore.success('Transaction created');
        return newTransaction;
      } catch (error) {
        console.error('Failed to create transaction:', error);
        notificationStore.error('Failed to create transaction');
        throw error;
      }
    },

    async updateTransaction(id: string, transaction: Partial<Transaction>) {
      try {
        const updated = await transactionRepository.update(id, transaction);
        update(transactions =>
          transactions.map(t => t.id === id ? updated : t)
        );
        notificationStore.success('Transaction updated');
        return updated;
      } catch (error) {
        console.error('Failed to update transaction:', error);
        notificationStore.error('Failed to update transaction');
        throw error;
      }
    },

    async remove(id: string) {
      try {
        await transactionRepository.delete(id);
        update(transactions => transactions.filter(t => t.id !== id));
        notificationStore.success('Transaction deleted');
      } catch (error) {
        console.error('Failed to delete transaction:', error);
        notificationStore.error('Failed to delete transaction');
        throw error;
      }
    },

    async removeMultiple(transactionIds: string[]) {
      try {
        const count = await transactionRepository.deleteMultiple(transactionIds);
        update(transactions =>
          transactions.filter(t => !transactionIds.includes(t.id))
        );
        notificationStore.success(`${count} transactions deleted`);
        return count;
      } catch (error) {
        console.error('Failed to delete transactions:', error);
        notificationStore.error('Failed to delete transactions');
        throw error;
      }
    },

    async bulkUpdateTags(transactionIds: string[], tagsToAdd?: string[], tagsToRemove?: string[]) {
      try {
        const count = await transactionRepository.bulkUpdateTags(transactionIds, tagsToAdd, tagsToRemove);
        // Reload transactions to reflect tag changes
        await this.load();
        notificationStore.success(`${count} transactions updated`);
        return count;
      } catch (error) {
        console.error('Failed to bulk update tags:', error);
        notificationStore.error('Failed to bulk update tags');
        throw error;
      }
    }
  };
}

export const transactionStore = createTransactionStore();