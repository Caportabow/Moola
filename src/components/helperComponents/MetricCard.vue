<script setup>
import { computed } from 'vue'

const props = defineProps({
  worst: String,
  best: String,
  average: String,
  labelName: String,
})

const averageFontSizeClass = computed(() => {
  if (props.average.length > 6) { 
    return 'text-l'
  } else if (props.average.length > 4) {
    return 'text-xl'
  }
  return 'text-2xl' // Default font size
})
</script>

<template>
  <div class="relative py-4 px-6 rounded-xl border border-muted bg-background">
    <div class="grid grid-cols-3 items-center text-center gap-x-8">
      <div class="flex flex-col items-center justify-center relative">
        <p class="text-[0.65rem] uppercase text-muted-foreground tracking-wider mb-1">Low</p>
        <p class="text-xl font-semibold text-red-500 leading-none">{{ worst }}</p>
        <span
          class="absolute -right-4 top-1/2 w-2 h-2 bg-red-500 rounded-full translate-y-[-50%]"
          aria-hidden="true"
        ></span>
      </div>

      <div class="relative flex flex-col items-center">
        <p class="text-[0.65rem] uppercase text-muted-foreground tracking-wider mb-1 z-10 bg-background px-2 relative">
          Average
        </p>

        <div class="flex items-baseline gap-1 z-10 relative">
          <p :class="averageFontSizeClass" class="font-extrabold leading-none">{{ average }}</p>
          <span
            class="text-xs text-muted-foreground uppercase tracking-wider pb-0.5 select-none"
          >
            {{ labelName }}
          </span>
        </div>
      </div>

      <div class="flex flex-col items-center justify-center relative">
        <p class="text-[0.65rem] uppercase text-muted-foreground tracking-wider mb-1">High</p>
        <p class="text-xl font-semibold text-green-500 leading-none">{{ best }}</p>
        <span
          class="absolute -left-4 top-1/2 w-2 h-2 bg-green-500 rounded-full translate-y-[-50%]"
          aria-hidden="true"
        ></span>
      </div>
    </div>
  </div>
</template>