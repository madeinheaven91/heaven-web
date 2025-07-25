<script lang="ts" setup>
import { ref, onMounted } from "vue";
import { scramble } from "../logic";

let title = ref("JazzCoding");

onMounted(() => {
  let t = "JazzCoding";
  void scramble(t, title);
  setInterval(async () => {
    await scramble(t, title);
  }, 15000);
});
</script>

<template>
  <a href="/blog">
    {{ title }}
  </a>
</template>

<style lang="css" scoped>
a{
  text-decoration: none;
  text-shadow: none;
  color: var(--main);
  font-weight: bold;
  cursor: pointer;
  font-size: 1.5em;
  padding: 0;
  border-bottom-width: 2px;
  transition: 0.3s;
  border-bottom-color: transparent;
}
a:hover{
  border-bottom-color: var(--main);
}
</style>
