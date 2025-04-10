<script setup lang="ts">
import { type Tag } from '@/entities/tag';

const props = defineProps<{ tag: Tag, clickable: Boolean }>();
</script>

<template>
  <div class="px-2 rounded-md text-center text-sm lg:text-base"  :style="{ backgroundColor: `#${props.tag.background_color}`, color: `#${props.tag.foreground_color}` }">
    <a v-if="props.clickable" :href="`./?tag=${props.tag.slug}`" :style="{ color: `#${props.tag.foreground_color}` }">
      {{ props.tag.name }}
    </a>
    <template v-else>
      {{ props.tag.name }}
    </template>
  </div>
</template>

<style scoped lang="css">
a{
  text-decoration: none;
  text-shadow: none;
}
</style>
