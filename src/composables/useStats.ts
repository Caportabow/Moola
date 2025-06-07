import { ref } from "vue";
import { invoke } from '@tauri-apps/api/core'
import type { HistoricalRates, CurrentRate, Stats, Trend } from "@/types";

export function useStats() {
  const loading = ref(false);
  const stats = ref<Stats | null>(null);

  async function loadStats(from: string, to: string, month: string, year: string) {
    if (!from || !to || !month || !year) return;
    if (from === to) return;

    loading.value = true;

    const [historical, current] = await Promise.all([
      invoke<HistoricalRates>("parse_exchange_rates", {
        from,
        to,
        year: year.toString(),
        month,
      }),
      invoke<CurrentRate>("get_current_exchange_rate", { from, to }),
    ]);

    const avg = historical.monthly?.average ?? 0;
    const currentRate = current.rate ?? 0;

    const trend: Trend =
      avg > currentRate ? "down" : avg < currentRate ? "up" : "neutral";

    const format = (n?: number) => (n != null ? n.toFixed(2) : "N/A");

    const date = new Date(current.date);
    const utcOffset = -date.getTimezoneOffset() / 60;
    const utcSign = utcOffset >= 0 ? '+' : '';
    const utcString = `UTC${utcSign}${utcOffset}`;

    stats.value = {
      avgMTD: format(historical.monthly?.average),
      avgYTD: format(historical.yearly?.average),
      bestYTD: format(historical.yearly?.best),
      worstYTD: format(historical.yearly?.worst),
      bestMTD: format(historical.monthly?.best),
      worstMTD: format(historical.monthly?.worst),
      current: currentRate.toFixed(2),
      updatedAt: `${date.toLocaleString()} (${utcString})`,
      trend,
    };

    loading.value = false;
  }

  return { stats, loading, loadStats };
}