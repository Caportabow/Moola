import currencyFlags from './currency-flags.json'
import type { CurrencyInfo } from "@/types";

export function useCurrencyName(rate: CurrencyInfo) {
  const flag = currencyFlags[rate.code.toUpperCase()] ?? '🏳️'
  return `${flag} ${rate.name} (${rate.code.toUpperCase()})`
}