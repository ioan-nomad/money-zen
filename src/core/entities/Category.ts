export interface Category {
  id: string;
  name: string;
  color: string;
  icon: string;
  category_type: 'income' | 'expense';
  created_at: string;
}