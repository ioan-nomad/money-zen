// APP CONFIG
export const APP_NAME = 'MoneyZen';
export const APP_VERSION = '1.0.0';
export const APP_DESCRIPTION = 'Modern Finance Management';

// DATABASE
export const DB_NAME = 'money-zen.db';
export const DB_VERSION = 1;

// UI CONSTANTS
export const TOAST_DURATION = 5000;
export const MODAL_ANIMATION_DURATION = 300;
export const DEBOUNCE_DELAY = 300;
export const PAGE_SIZE = 50;

// VALIDATION
export const MIN_ACCOUNT_NAME_LENGTH = 2;
export const MAX_ACCOUNT_NAME_LENGTH = 50;
export const MIN_DESCRIPTION_LENGTH = 0;
export const MAX_DESCRIPTION_LENGTH = 500;
export const MIN_AMOUNT = 0.01;
export const MAX_AMOUNT = 999999999.99;

// DATE FORMATS
export const DATE_FORMAT = 'yyyy-MM-dd';
export const DATETIME_FORMAT = 'yyyy-MM-dd HH:mm:ss';
export const DISPLAY_DATE_FORMAT = 'dd.MM.yyyy';

// CURRENCY
export const DEFAULT_CURRENCY = 'RON';
export const SUPPORTED_CURRENCIES = ['RON', 'EUR', 'USD', 'GBP'];

// TRANSACTION TYPES
export const TRANSACTION_TYPES = {
  INCOME: 'income',
  EXPENSE: 'expense'
} as const;

// ERROR MESSAGES
export const ERROR_MESSAGES = {
  GENERIC: 'Ceva nu a mers bine. Încearcă din nou.',
  NETWORK: 'Probleme de conexiune. Verifică internetul.',
  VALIDATION: 'Verifică datele introduse.',
  NOT_FOUND: 'Resursa nu a fost găsită.',
  UNAUTHORIZED: 'Nu ai permisiuni pentru această acțiune.'
};

// SUCCESS MESSAGES
export const SUCCESS_MESSAGES = {
  SAVED: 'Salvat cu succes!',
  DELETED: 'Șters cu succes!',
  UPDATED: 'Actualizat cu succes!',
  IMPORTED: 'Import realizat cu succes!'
};
