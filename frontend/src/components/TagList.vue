<script setup lang="ts">
import { ref, onMounted } from 'vue';
// import TagBadge from './TagBadge.vue';

const props = defineProps({
  tags: Array,
  fallback: Boolean
});

onMounted(() => {
})
</script>

<template>
  <div v-if="tags && tags.length">
    <div v-for="tag in tags" :key="tag.slug">
      <a class="badge mr-1" :style="{ 
          textDecoration: 'none',
          backgroundColor: '#' + tag.background_color,
          color: '#' + tag.foreground_color
      }">
        {{ tag.name }}
      </a>
    </div>
  </div>
  <p v-else-if="fallback" class="fs-3 text-secondary">Нет тегов...</p>
</template>

<style>
</style>
