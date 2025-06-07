import rawCurrencyFlags from './currency-flags.json'
import type { CurrencyInfo, CurrencyFlags } from "@/types";

const currencyFlags: CurrencyFlags = rawCurrencyFlags;
export function useCurrencyName(rate: CurrencyInfo) {
  const code = rate.code.toUpperCase();
  const flag = currencyFlags[code] ?? '🏳️';
  return `${flag} ${rate.name} (${code})`;
}