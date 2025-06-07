import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { CurrencyInfo } from "@/types";

export function useCurrency() {
  const rates = ref<CurrencyInfo[]>([])

  const loadRates = async () => {
    rates.value = await invoke<CurrencyInfo[]>('get_available_currency_codes')
  }

  onMounted(loadRates)

  return {
    rates,
  }
}