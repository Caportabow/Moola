<script setup lang="ts">
import ThemeToggler from '@/components/helperComponents/ThemeToggler.vue'
import InputField from '@/components/InputField.vue'
import ExchangeStats from '@/components/ExchangeStats.vue'
import { ref } from 'vue'
import { TrendingUp } from 'lucide-vue-next'

const monthLabels = [
  'January', 'February', 'March', 'April', 'May', 'June',
  'July', 'August', 'September', 'October', 'November', 'December'
]

const from = ref('')
const to = ref('')
const month = ref(monthLabels[new Date().getMonth()])
const year = ref(new Date().getFullYear().toString())
</script>

<template>
  <div class="mx-auto max-w-4xl p-6 space-y-6">
    <!-- Header Block -->
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-bold tracking-tight text-foreground">
          Moola
        </h1>
      </div>
      <ThemeToggler />
    </div>

    <!-- Input Field Block -->
    <InputField
      :monthLabels="monthLabels"
      v-model:modelValueFrom="from"
      v-model:modelValueTo="to"
      v-model:modelValueMonth="month"
      v-model:modelValueYear="year"
    />

    <!-- Stats Output -->
    <ExchangeStats :from="from" :to="to" :month="month" :year="year" />
  </div>
</template>