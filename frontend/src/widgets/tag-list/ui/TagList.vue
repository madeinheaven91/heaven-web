<script setup lang="ts">
import { type Tag } from '@/entities/tag'
import TagBadge from './TagBadge.vue'

const props = defineProps({
  tags: Array<Tag>,
  clickable: Boolean
});
</script>

<template>
  <div class="flex flex-row gap-2">
    <TagBadge v-for="tag in props.tags" :tag="tag" :clickable="props.clickable" />
  </div>
</template>

<style lang="css" scoped>
</style>
