export interface CurrencyInfo {
  name: string
  code: string
}

export type HistoricalRates = {
  monthly?: {
    average: number;
    best: number;
    worst: number;
  };
  yearly?: {
    average: number;
    best: number;
    worst: number;
  };
};

export type CurrentRate = {
  rate: number;
  date: string;
};

export type Trend = "up" | "down" | "neutral";

export type Stats = {
  avgMTD: string;
  avgYTD: string;
  bestYTD: string;
  worstYTD: string;
  bestMTD: string;
  worstMTD: string;
  current: string;
  updatedAt: string;
  trend: Trend;
};

export type CurrencyFlags = Record<string, string>;