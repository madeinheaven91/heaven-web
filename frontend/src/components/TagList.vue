<script setup lang="ts">
import type { Tag } from '../shared/models.ts' 

const props = defineProps<{
  tags: Tag[],
  fallback?: Boolean
}>();

</script>

<template>
  <div v-if="props.tags && props.tags.length">
    <div v-for="tag in props.tags" :key="tag.slug">
      <a class="badge mr-1" :style="{ 
          textDecoration: 'none',
          backgroundColor: '#' + tag.background_color,
          color: '#' + tag.foreground_color
      }">
        {{ tag.name }}
      </a>
    </div>
  </div>
  <p v-else-if="props.fallback" class="fs-3 text-secondary">Нет тегов...</p>
</template>

<style>
</style>
