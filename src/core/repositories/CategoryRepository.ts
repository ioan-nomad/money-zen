// CategoryRepository.ts - CRUD pentru Categories
import { invoke } from '@tauri-apps/api/core';
import type { Category } from '../entities/Category';

class CategoryRepository {
  async getAll(): Promise<Category[]> {
    try {
      return await invoke('get_categories');
    } catch (error) {
      console.error('CategoryRepository.getAll error:', error);
      throw new Error('Failed to fetch categories');
    }
  }

  async create(category: Omit<Category, 'id' | 'created_at' | 'updated_at'>): Promise<Category> {
    try {
      return await invoke('create_category', {
        name: category.name,
        icon: category.icon,
        categoryType: category.category_type,
        color: category.color
      });
    } catch (error) {
      console.error('CategoryRepository.create error:', error);
      throw new Error('Failed to create category');
    }
  }

  async update(id: string, category: Partial<Category>): Promise<Category> {
    try {
      return await invoke('update_category', {
        id,
        name: category.name,
        icon: category.icon,
        categoryType: category.category_type,
        color: category.color
      });
    } catch (error) {
      console.error('CategoryRepository.update error:', error);
      throw new Error('Failed to update category');
    }
  }

  async delete(id: string): Promise<void> {
    try {
      await invoke('delete_category', { id });
    } catch (error) {
      console.error('CategoryRepository.delete error:', error);
      throw new Error('Failed to delete category');
    }
  }
}

export const categoryRepository = new CategoryRepository();