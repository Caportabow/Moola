<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useCurrency } from '@/composables/useCurrency'
import { useCurrencyName } from '@/composables/useCurrencyName'

import { ArrowLeftRight } from 'lucide-vue-next'

import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem
} from '@/components/ui/select'
import { Skeleton } from '@/components/ui/skeleton'
import { Button } from '@/components/ui/button'

const props = defineProps<{
  monthLabels: string[]
  modelValueFrom: string
  modelValueTo: string
  modelValueMonth: string
  modelValueYear: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValueFrom', value: string): void
  (e: 'update:modelValueTo', value: string): void
  (e: 'update:modelValueMonth', value: string): void
  (e: 'update:modelValueYear', value: string): void
}>()

// Подключаем currencies
const { rates } = useCurrency()

const from = ref(props.modelValueFrom)
const to = ref(props.modelValueTo)
const fromOptions = computed(() => rates.value.filter(r => r.code !== to.value))
const toOptions = computed(() => rates.value.filter(r => r.code !== from.value))
const selectedMonth = ref(props.modelValueMonth || props.monthLabels[new Date().getMonth()])
const currentYear = new Date().getFullYear()
const years = Array.from({ length: 10 }, (_, i) => String(currentYear - i))
const selectedYear = ref(props.modelValueYear || years[0])

// Обновление родителя
watch(from, val => emit('update:modelValueFrom', val))
watch(to, val => emit('update:modelValueTo', val))
watch(selectedYear, val => emit('update:modelValueYear', val))
watch(selectedMonth, val => emit('update:modelValueMonth', val))

// Перерасчёт monthLabels чтобы мы не показывали месяца в будущем
const filteredMonthLabels = computed(() => {
  if (selectedYear.value === String(currentYear)) {
    return props.monthLabels.slice(0, new Date().getMonth() + 1)
  }
  return props.monthLabels
})

// Следим за filteredMonthLabels и корректируем выбранный месяц
watch(filteredMonthLabels, months => {
  if (!months.includes(selectedMonth.value)) {
    const lastAvailable = months[months.length - 1]
    if (selectedMonth.value !== lastAvailable) {
      selectedMonth.value = lastAvailable
    }
  }
})
</script>

<template>
  <div class="flex flex-col md:flex-row gap-3 items-end w-full p-2 rounded-xl border border-muted shadow-md">
    <div class="flex flex-col md:flex-row gap-1.5 w-full items-end">
      <!-- From -->
      <div class="w-64 flex-1">
        <label class="text-xs text-muted-foreground mb-1 block">From</label>
        <Select v-if="rates.length" v-model="from">
          <SelectTrigger class="w-full h-10 px-3 rounded-xl border-muted text-sm flex items-center">
            <SelectValue class="truncate" placeholder="Select origin" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem class="truncate" v-for="rate in fromOptions" :key="rate.name" :value="rate.code">
              {{ useCurrencyName(rate) }}
            </SelectItem>
          </SelectContent>
        </Select>
        <Skeleton v-else class="h-9 rounded-xl" />
      </div>

      <!-- Swap Button -->
      <Button
        variant="outline"
        class="h-9 w-9 rounded-xl border-muted flex items-center justify-center"
        @click="([from, to] = [to, from])"
        :disabled="!from || !to"
      >
        <ArrowLeftRight class="h-5 w-5 text-muted-foreground" />
      </Button>

      <!-- To -->
      <div class="w-40 flex-1">
        <label class="text-xs text-muted-foreground mb-1 block">To</label>
        <Select v-if="rates.length" v-model="to">
          <SelectTrigger class="w-full h-10 px-3 rounded-xl border-muted text-sm flex items-center">
            <SelectValue class="truncate" placeholder="Select target" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem class="truncate" v-for="rate in toOptions" :key="rate.name" :value="rate.code">
              {{ useCurrencyName(rate) }}
            </SelectItem>
          </SelectContent>
        </Select>
        <Skeleton v-else class="h-9 rounded-xl" />
      </div>
    </div>

    <!-- Month & Year -->
    <div class="flex gap-2">
      <!-- Month -->
      <div class="w-30">
        <label class="text-xs text-muted-foreground mb-1 block">Month</label>
        <Select v-model="selectedMonth">
          <SelectTrigger class="w-full h-10 rounded-xl border-muted text-sm">
            <SelectValue placeholder="Month" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem v-for="m in filteredMonthLabels" :key="m" :value="m">
              {{ m }}
            </SelectItem>
          </SelectContent>
        </Select>
      </div>

      <!-- Year -->
      <div class="w-25">
        <label class="text-xs text-muted-foreground mb-1 block">Year</label>
        <Select v-model="selectedYear">
          <SelectTrigger class="w-full h-10 rounded-xl border-muted text-sm">
            <SelectValue placeholder="Year" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem v-for="y in years" :key="y" :value="y">
              {{ y }}
            </SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>
  </div>
</template>
