// categoryStore.ts - State management pentru Categories
import { writable } from 'svelte/store';
import { categoryRepository } from '../../core/repositories/CategoryRepository';
import type { Category } from '../../core/entities/Category';

function createCategoryStore() {
  const { subscribe, set, update } = writable<Category[]>([]);

  return {
    subscribe,

    async load() {
      try {
        const categories = await categoryRepository.getAll();
        set(categories);
      } catch (error) {
        console.error('Failed to load categories:', error);
        // notificationStore.error('Failed to load categories');
      }
    },

    async create(category: Omit<Category, 'id' | 'created_at' | 'updated_at'>) {
      try {
        const newCategory = await categoryRepository.create(category);
        update(categories => [...categories, newCategory]);
        // notificationStore.success('Category created');
        return newCategory;
      } catch (error) {
        console.error('Failed to create category:', error);
        // notificationStore.error('Failed to create category');
        throw error;
      }
    },

    async updateCategory(id: string, category: Partial<Category>) {
      try {
        const updated = await categoryRepository.update(id, category);
        update(categories =>
          categories.map(c => c.id === id ? updated : c)
        );
        // notificationStore.success('Category updated');
        return updated;
      } catch (error) {
        console.error('Failed to update category:', error);
        // notificationStore.error('Failed to update category');
        throw error;
      }
    },

    async remove(id: string) {
      try {
        await categoryRepository.delete(id);
        update(categories => categories.filter(c => c.id !== id));
        // notificationStore.success('Category deleted');
      } catch (error) {
        console.error('Failed to delete category:', error);
        // notificationStore.error('Failed to delete category');
        throw error;
      }
    }
  };
}

export const categoryStore = createCategoryStore();