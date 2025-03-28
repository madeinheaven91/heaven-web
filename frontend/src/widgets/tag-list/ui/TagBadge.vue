<script setup lang="ts">
import { type Tag } from '@/entities/tag';

const props = defineProps<{ tag: Tag, clickable: Boolean }>();

</script>

<template>
  <a v-if="props.clickable" class="tag" :href="`./?tag=${props.tag.slug}`" :style="{ color: `#${props.tag.background_color}` }">
    {{ props.tag.name }}
  </a>
  <span v-else class="tag" :style="{ color: `#${props.tag.background_color}` }">
    {{ props.tag.name }}
  </span>
</template>

<style scoped lang="css">
a{
  text-decoration: none;
  text-shadow: none;
}
.tag::after{
  content: ")";
  padding-left: var(--small-l);
}
.tag::before{
  content: "(";
  padding-right: var(--small-l);
}
</style>
