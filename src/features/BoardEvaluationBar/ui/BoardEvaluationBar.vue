<template>
  <div class="eval-container">
    <div class="eval-bar">
      <div
        class="eval-fill"
        :style="{
          height: fillHeight + '%',
        }"
      />
    </div>

    <div class="eval-label">
      {{ formattedEval }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

// ---------------- PROPS ----------------
const props = defineProps<{
  evalCp: number | null;
}>();

// ---------------- NORMALIZATION ----------------

// ограничим диапазон
const clamped = computed(() => {
  if (props.evalCp === null) return 0;

  return Math.max(-1000, Math.min(1000, props.evalCp));
});

// переводим в %
const fillHeight = computed(() => {
  // 0 = 50%
  // +1000 = 100%
  // -1000 = 0%

  return ((clamped.value + 1000) / 2000) * 100;
});

// ---------------- LABEL ----------------
const formattedEval = computed(() => {
  if (props.evalCp === null) return '0.0';

  if (Math.abs(props.evalCp) >= 10000) {
    return props.evalCp > 0 ? 'M+' : 'M-';
  }

  return (props.evalCp / 100).toFixed(1);
});
</script>

<style scoped>
.eval-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

/* сам бар */
.eval-bar {
  width: 20px;
  height: 520px;
  background: black;
  position: relative;
  border-radius: 6px;
  overflow: hidden;
}

/* белая часть */
.eval-fill {
  position: absolute;
  bottom: 0;
  width: 100%;
  background: white;
  transition: height 0.3s ease;
}

/* подпись */
.eval-label {
  font-size: 12px;
  color: #888;
}
</style>
