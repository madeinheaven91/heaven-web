<script setup lang="ts">

</script>

<template>
  <span>
    <slot />
  </span>
</template>

<style lang="css" scoped>
span::before{
  content: '[';
  padding-right: 2px;
}
span::after{
  content: ']';
  padding-left: 2px;
}
span {
  cursor: pointer;
  border-bottom-width: 2px;
  border-bottom-color: transparent;
  transition: 0.3s;
}

span:hover {
  border-bottom-color: var(--main);
}

span:active {
}
</style>
