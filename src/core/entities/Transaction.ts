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
  tags?: string[];
}
