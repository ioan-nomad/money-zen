// tagStore.ts - State management pentru Tags
import { writable } from 'svelte/store';
import { tagRepository } from '../../core/repositories/TagRepository';
import type { Tag } from '../../core/entities/Tag';

function createTagStore() {
  const { subscribe, set, update } = writable<Tag[]>([]);

  return {
    subscribe,

    async load() {
      try {
        const tags = await tagRepository.getAll();
        set(tags);
      } catch (error) {
        console.error('Failed to load tags:', error);
        // notificationStore.error('Failed to load tags');
      }
    },

    async loadByTransaction(transactionId: string) {
      try {
        const tags = await tagRepository.getByTransaction(transactionId);
        return tags;
      } catch (error) {
        console.error('Failed to load transaction tags:', error);
        // notificationStore.error('Failed to load transaction tags');
        return [];
      }
    },

    async create(tag: Omit<Tag, 'id' | 'created_at' | 'updated_at'>) {
      try {
        const newTag = await tagRepository.create(tag);
        update(tags => [...tags, newTag]);
        // notificationStore.success('Tag created');
        return newTag;
      } catch (error) {
        console.error('Failed to create tag:', error);
        // notificationStore.error('Failed to create tag');
        throw error;
      }
    },

    async updateTag(id: string, tag: Partial<Tag>) {
      try {
        const updated = await tagRepository.update(id, tag);
        update(tags =>
          tags.map(t => t.id === id ? updated : t)
        );
        // notificationStore.success('Tag updated');
        return updated;
      } catch (error) {
        console.error('Failed to update tag:', error);
        // notificationStore.error('Failed to update tag');
        throw error;
      }
    },

    async remove(id: string) {
      try {
        await tagRepository.delete(id);
        update(tags => tags.filter(t => t.id !== id));
        // notificationStore.success('Tag deleted');
      } catch (error) {
        console.error('Failed to delete tag:', error);
        // notificationStore.error('Failed to delete tag');
        throw error;
      }
    },

    async addToTransaction(transactionId: string, tagIds: string[]) {
      try {
        await tagRepository.addToTransaction(transactionId, tagIds);
        // notificationStore.success('Tags added to transaction');
      } catch (error) {
        console.error('Failed to add tags to transaction:', error);
        // notificationStore.error('Failed to add tags');
        throw error;
      }
    },

    async removeFromTransaction(transactionId: string, tagIds: string[]) {
      try {
        await tagRepository.removeFromTransaction(transactionId, tagIds);
        // notificationStore.success('Tags removed from transaction');
      } catch (error) {
        console.error('Failed to remove tags from transaction:', error);
        // notificationStore.error('Failed to remove tags');
        throw error;
      }
    }
  };
}

export const tagStore = createTagStore();