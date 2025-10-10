// TagRepository.ts - CRUD pentru Tags
import { invoke } from '@tauri-apps/api/core';
import type { Tag } from '../entities/Tag';

class TagRepository {
  async getAll(): Promise<Tag[]> {
    try {
      return await invoke('get_tags');
    } catch (error) {
      console.error('TagRepository.getAll error:', error);
      throw new Error('Failed to fetch tags');
    }
  }

  async getByTransaction(transactionId: string): Promise<Tag[]> {
    try {
      return await invoke('get_transaction_tags', { transactionId });
    } catch (error) {
      console.error('TagRepository.getByTransaction error:', error);
      throw new Error('Failed to fetch transaction tags');
    }
  }

  async getTransactionsByTag(tagId: string): Promise<string[]> {
    try {
      return await invoke('get_transactions_by_tag', { tagId });
    } catch (error) {
      console.error('TagRepository.getTransactionsByTag error:', error);
      throw new Error('Failed to fetch transactions by tag');
    }
  }

  async create(tag: Omit<Tag, 'id' | 'created_at' | 'updated_at'>): Promise<Tag> {
    try {
      return await invoke('create_tag', {
        name: tag.name,
        icon: tag.icon || '',
        color: tag.color
      });
    } catch (error) {
      console.error('TagRepository.create error:', error);
      throw new Error('Failed to create tag');
    }
  }

  async update(id: string, tag: Partial<Tag>): Promise<Tag> {
    try {
      return await invoke('update_tag', {
        id,
        name: tag.name,
        icon: tag.icon || '',
        color: tag.color
      });
    } catch (error) {
      console.error('TagRepository.update error:', error);
      throw new Error('Failed to update tag');
    }
  }

  async delete(id: string): Promise<void> {
    try {
      await invoke('delete_tag', { id });
    } catch (error) {
      console.error('TagRepository.delete error:', error);
      throw new Error('Failed to delete tag');
    }
  }

  async addToTransaction(transactionId: string, tagIds: string[]): Promise<void> {
    try {
      await invoke('add_tags_to_transaction', { transactionId, tagIds });
    } catch (error) {
      console.error('TagRepository.addToTransaction error:', error);
      throw new Error('Failed to add tags to transaction');
    }
  }

  async removeFromTransaction(transactionId: string, tagIds: string[]): Promise<void> {
    try {
      await invoke('remove_tags_from_transaction', { transactionId, tagIds });
    } catch (error) {
      console.error('TagRepository.removeFromTransaction error:', error);
      throw new Error('Failed to remove tags from transaction');
    }
  }
}

export const tagRepository = new TagRepository();