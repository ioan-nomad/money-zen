// src/core/services/PDFParserService.ts
import type { Transaction } from '../entities/Transaction';

interface ParsedTransaction {
  date: string;
  description: string;
  amount: number;
  type: 'income' | 'expense';
  suggestedCategory?: string;
  confidence?: number;
}

interface BankPattern {
  name: string;
  identifiers: string[];
  dateRegex: RegExp;
  transactionRegex: RegExp;
  parseTransaction: (match: RegExpMatchArray) => ParsedTransaction | null;
}

export class PDFParserService {
  private static instance: PDFParserService;

  // Pattern-uri învățate din utilizare
  private learnedPatterns: Map<string, string> = new Map();

  // Categorii predefinite bazate pe cuvinte cheie
  private categoryKeywords = {
    'Mâncare': ['kaufland', 'lidl', 'mega', 'carrefour', 'auchan', 'profi', 'penny', 'glovo', 'foodpanda', 'tazz'],
    'Transport': ['omv', 'petrom', 'mol', 'rompetrol', 'lukoil', 'uber', 'bolt', 'stb', 'metrorex'],
    'Utilități': ['enel', 'engie', 'eon', 'digi', 'orange', 'vodafone', 'telekom', 'apa nova'],
    'Divertisment': ['netflix', 'spotify', 'hbo', 'cinema', 'teatru', 'steam', 'playstation'],
    'Sănătate': ['farmacia', 'catena', 'helpnet', 'sensiblu', 'medlife', 'regina maria'],
    'Shopping': ['emag', 'altex', 'flanco', 'dedeman', 'ikea', 'h&m', 'zara', 'decathlon'],
    'Restaurant': ['restaurant', 'pizzeria', 'mcdonalds', 'kfc', 'starbucks', 'tucano'],
    'Transfer': ['transfer', 'revolut', 'paypal', 'wise'],
    'Cash': ['atm', 'retragere', 'numerar', 'bancomat'],
    'Salariu': ['salariu', 'salary', 'venit', 'income']
  };

  // Pattern-uri pentru bănci românești
  private bankPatterns: BankPattern[] = [
    {
      name: 'BT',
      identifiers: ['BANCA TRANSILVANIA', 'BT24', 'Banca Transilvania S.A.'],
      dateRegex: /(\d{2}[./-]\d{2}[./-]\d{4})/,
      transactionRegex: /(\d{2}[./-]\d{2}[./-]\d{4})\s+(.*?)\s+(-?\d+[.,]\d{2})\s+RON/g,
      parseTransaction: (match) => {
        const [_, date, description, amount] = match;
        const numericAmount = parseFloat(amount.replace(',', '.'));
        return {
          date: this.normalizeDate(date),
          description: description.trim(),
          amount: Math.abs(numericAmount),
          type: numericAmount < 0 ? 'expense' : 'income'
        };
      }
    },
    {
      name: 'BCR',
      identifiers: ['BCR', 'Banca Comercială Română'],
      dateRegex: /(\d{2}\/\d{2}\/\d{4})/,
      transactionRegex: /(\d{2}\/\d{2}\/\d{4})\s+(.*?)\s+(-?\d+(?:\.\d{3})*,\d{2})/g,
      parseTransaction: (match) => {
        const [_, date, description, amount] = match;
        const numericAmount = parseFloat(
          amount.replace(/\./g, '').replace(',', '.')
        );
        return {
          date: this.normalizeDate(date),
          description: description.trim(),
          amount: Math.abs(numericAmount),
          type: numericAmount < 0 ? 'expense' : 'income'
        };
      }
    },
    {
      name: 'ING',
      identifiers: ['ING Bank', 'ING Home Bank'],
      dateRegex: /(\d{2}-\d{2}-\d{4})/,
      transactionRegex: /(\d{2}-\d{2}-\d{4})\s+(.*?)\s+(-?\d+(?:,\d{3})*\.\d{2})/g,
      parseTransaction: (match) => {
        const [_, date, description, amount] = match;
        const numericAmount = parseFloat(
          amount.replace(/,/g, '')
        );
        return {
          date: this.normalizeDate(date),
          description: description.trim(),
          amount: Math.abs(numericAmount),
          type: numericAmount < 0 ? 'expense' : 'income'
        };
      }
    }
  ];

  private constructor() {
    this.loadLearnedPatterns();
  }

  static getInstance(): PDFParserService {
    if (!PDFParserService.instance) {
      PDFParserService.instance = new PDFParserService();
    }
    return PDFParserService.instance;
  }

  async parsePDF(file: File): Promise<ParsedTransaction[]> {
    try {
      const arrayBuffer = await file.arrayBuffer();
      const pdfData = new Uint8Array(arrayBuffer);

      // Folosim pdf-parse pentru a extrage textul
      const pdf = await this.extractTextFromPDF(pdfData);

      // Identificăm banca
      const bank = this.identifyBank(pdf.text);
      if (!bank) {
        throw new Error('Format de extras nerecunoscut. Verificați că este un extras de la BT, BCR sau ING.');
      }

      // Extragem tranzacțiile
      const transactions = this.extractTransactions(pdf.text, bank);

      // Adăugăm sugestii de categorizare
      return transactions.map(tx => ({
        ...tx,
        ...this.suggestCategory(tx.description)
      }));

    } catch (error) {
      console.error('Error parsing PDF:', error);
      throw new Error('Eroare la procesarea PDF-ului: ' + error.message);
    }
  }

  private async extractTextFromPDF(data: Uint8Array): Promise<any> {
    // Simulăm extragerea - în producție ai folosi pdf-parse real
    // Pentru Tauri, va trebui să folosim un backend Rust sau o librărie WASM
    return {
      text: `Sample bank statement text`,
      numPages: 1
    };
  }

  private identifyBank(text: string): BankPattern | null {
    const upperText = text.toUpperCase();
    for (const pattern of this.bankPatterns) {
      if (pattern.identifiers.some(id => upperText.includes(id.toUpperCase()))) {
        return pattern;
      }
    }
    return null;
  }

  private extractTransactions(text: string, bank: BankPattern): ParsedTransaction[] {
    const transactions: ParsedTransaction[] = [];
    let match;

    while ((match = bank.transactionRegex.exec(text)) !== null) {
      const parsed = bank.parseTransaction(match);
      if (parsed) {
        transactions.push(parsed);
      }
    }

    return transactions;
  }

  private suggestCategory(description: string): { suggestedCategory?: string; confidence?: number } {
    const lowerDesc = description.toLowerCase();

    // Verifică pattern-uri învățate
    const learned = this.learnedPatterns.get(lowerDesc);
    if (learned) {
      return { suggestedCategory: learned, confidence: 100 };
    }

    // Verifică cuvinte cheie
    for (const [category, keywords] of Object.entries(this.categoryKeywords)) {
      for (const keyword of keywords) {
        if (lowerDesc.includes(keyword)) {
          return {
            suggestedCategory: category,
            confidence: 80
          };
        }
      }
    }

    // Verifică pattern-uri comune
    if (lowerDesc.match(/restaurant|pizza|burger|kebab/)) {
      return { suggestedCategory: 'Restaurant', confidence: 70 };
    }

    if (lowerDesc.match(/taxi|uber|bolt/)) {
      return { suggestedCategory: 'Transport', confidence: 70 };
    }

    return {};
  }

  private normalizeDate(dateStr: string): string {
    // Convertește diferite formate de dată la YYYY-MM-DD
    const parts = dateStr.split(/[./-]/);
    if (parts.length === 3) {
      const [day, month, year] = parts;
      return `${year}-${month.padStart(2, '0')}-${day.padStart(2, '0')}`;
    }
    return dateStr;
  }

  learnPattern(description: string, category: string): void {
    this.learnedPatterns.set(description.toLowerCase(), category);
    this.saveLearnedPatterns();
  }

  private loadLearnedPatterns(): void {
    const saved = localStorage.getItem('pdf-import-patterns');
    if (saved) {
      try {
        const patterns = JSON.parse(saved);
        this.learnedPatterns = new Map(patterns);
      } catch (e) {
        console.error('Failed to load learned patterns');
      }
    }
  }

  private saveLearnedPatterns(): void {
    const patterns = Array.from(this.learnedPatterns.entries());
    localStorage.setItem('pdf-import-patterns', JSON.stringify(patterns));
  }
}