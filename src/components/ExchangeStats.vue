<script setup lang="ts">
import { watch, computed } from "vue";
import { Minus, ArrowDown, ArrowUp, ChartNoAxesCombined } from "lucide-vue-next"; // UI импорты
import { Skeleton } from '@/components/ui/skeleton'
import MetricCard from './helperComponents/MetricCard.vue'
import EmptyNotifier from './helperComponents/EmptyNotifier.vue'
import { useStats } from "@/composables/useStats";

const props = defineProps<{
  from: string;
  to: string;
  year: string;
  month: string;
}>();

const { stats, loading, loadStats } = useStats();

watch(
  () => ({ ...props }),
  ({ from, to, month, year }) => {
    loadStats(from, to, month, year);
  },
  { immediate: true }
);

const trendColor = computed(() =>
  stats.value?.trend === "up"
    ? "text-green-600"
    : stats.value?.trend === "down"
    ? "text-red-600"
    : "text-muted-foreground"
);
const trendIcon = computed(() =>
  stats.value?.trend === 'up' ? ArrowUp
  : stats.value?.trend === 'down' ? ArrowDown
  : Minus
);
</script>

<template>
    <div v-if="loading || stats" class="mt-6 p-6 border border-muted rounded-2xl bg-card space-y-6 shadow-md">
        <!-- Header -->
        <div class="flex items-center gap-3">
            <ChartNoAxesCombined class="w-6 h-6 text-primary" />
            <h3 class="text-lg font-semibold">Exchange Rate Insights</h3>
            <div v-if="stats" class="ml-auto text-xs text-muted-foreground flex items-center gap-1.5">
                <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                Updated {{ stats.updatedAt }}
            </div>
        </div>
  
        <!-- Loading State -->
        <div v-if="loading" class="space-y-6">
            <Skeleton class="h-29 rounded-xl" />
            <div class="flex gap-4">
                <Skeleton class="h-20 flex-1 rounded-xl" />
                <Skeleton class="h-20 flex-1 rounded-xl" />
            </div>
        </div>
  
        <!-- Content -->
        <div v-else-if="stats" class="space-y-6">
            <!-- Current Rate Spotlight -->
            <div class="relative overflow-hidden p-6 rounded-xl bg-gradient-to-br from-card to-muted/5 border border-muted">
                <!-- Background trend graph -->
                <div>
                    <div
                        class="absolute inset-0 z-0 pointer-events-none opacity-5"
                        :class="{
                            'bg-[linear-gradient(45deg,theme(colors.green.500)_0%,transparent_100%)]': stats.trend === 'up',
                            'bg-[linear-gradient(-45deg,theme(colors.red.500)_0%,transparent_100%)]': stats.trend === 'down'
                        }"
                    />
                    <svg v-if="stats.trend === 'up'"
                        class="absolute inset-0 w-full h-full z-0 opacity-10 pointer-events-none"
                        viewBox="0 0 100 100"
                        preserveAspectRatio="none"
                    >
                        <polyline
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            points="0,100 20,60 40,70 60,30 80,40 100,10"
                            class="text-green-500"
                        />
                    </svg>

                    <svg v-else-if="stats.trend === 'down'"
                        class="absolute inset-0 w-full h-full z-0 opacity-10 pointer-events-none"
                        viewBox="0 0 100 100"
                        preserveAspectRatio="none"
                    >
                        <polyline
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            points="0,10 20,40 40,30 60,70 80,60 100,100"
                            class="text-red-500"
                        />
                    </svg>
                </div>
                <!-- Header -->
                <div class="relative z-10">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-muted-foreground mb-1">Live Rate</p>
                            <div class="flex items-baseline gap-3">
                                <span class="text-4xl font-bold text-foreground">{{ stats.current }}</span>
                                <span :class="trendColor" class="flex items-center gap-1 text-sm font-medium">
                                    <component :is="trendIcon" :class="['w-4 h-4', trendColor]" />
                                    {{ stats.trend.toUpperCase() }}
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <!-- Metrics Dashboard -->
            <div class="grid gap-4 md:grid-cols-2">
                <!-- MTD Card -->
                <MetricCard 
                    :current="stats.current"
                    :average="stats.avgMTD"
                    :best="stats.bestMTD"
                    :worst="stats.worstMTD"
                    labelName="MTD"
                />
    
                <!-- YTD Card -->
                <MetricCard 
                    :current="stats.current"
                    :average="stats.avgYTD"
                    :best="stats.bestYTD"
                    :worst="stats.worstYTD"
                    labelName="YTD"
                />
            </div>
        </div>
    </div>
    <EmptyNotifier v-else />
</template>