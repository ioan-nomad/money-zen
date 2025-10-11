// Re-export all types from core entities for backwards compatibility
// This allows components to import from '$lib/types' as before
export type { Account } from '../core/entities/Account';
export type { Transaction } from '../core/entities/Transaction';
export type { Category } from '../core/entities/Category';
export type { Tag } from '../core/entities/Tag';