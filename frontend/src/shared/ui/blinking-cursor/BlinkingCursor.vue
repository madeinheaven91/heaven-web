<template>
<span class="blinking-cursor">_</span>
</template>

<style lang="css">
.blinking-cursor {
  animation: blink 1s infinite;
}
@keyframes blink {
  0% { opacity: 0; }
  49% { opacity: 0; }
  50% { opacity: 1; }
  99% { opacity: 1; }
  100% { opacity: 0; }
}
</style>
