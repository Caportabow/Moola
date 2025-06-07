<template>
  <div
    class="flex flex-col items-center justify-center h-[300px] gap-4 px-6 select-none"
    role="alert"
    aria-live="polite"
  >
    <svg
      class="w-24 h-16 stroke-muted-foreground animate-pulse"
      viewBox="0 0 100 60"
      fill="none"
      stroke-width="3"
      stroke-linejoin="round"
      stroke-linecap="round"
      aria-hidden="true"
      xmlns="http://www.w3.org/2000/svg"
    >
      <!-- Пульсирующая линия плейсхолдера графика -->
      <polyline
        points="5 50 20 40 40 45 60 30 80 35 95 20"
        stroke="currentColor"
        stroke-dasharray="150"
        stroke-dashoffset="150"
        >
        <animate
          attributeName="stroke-dashoffset"
          from="150"
          to="0"
          dur="2s"
          repeatCount="indefinite" />
      </polyline>
    </svg>

    <p class="text-sm text-muted-foreground text-center max-w-xs">
      Your input powers this chart.
    </p>
  </div>
</template>

<style>
/* анимация пульсации через Tailwind классы */
.animate-pulse {
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 0.5;
  }
  50% {
    opacity: 1;
  }
}
</style>