export interface Account {
  id: string;
  name: string;
  account_type: string;
  balance: number;
  currency: string;
  created_at: string;
  updated_at: string;
  owner?: string;
}
