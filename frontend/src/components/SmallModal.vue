<script lang="ts" setup>

const props = defineProps({
  title: String,
  id: String
});

</script>

<template>
  <div class="modal fade" :id="props.id" tabindex="-1" role="dialog" aria-labelledby="mySmallModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-sm">
      <div class="modal-content">
        <div class="modal-header d-flex justify-content-between">
          <h5 class="modal-title">{{ title }}</h5>
          <h2 type="button" class="close" data-bs-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">×</span>
          </h2>
        </div>
        <div class="modal-body">
          <slot>
          </slot>
        </div>
      </div>
    </div>
  </div>
</template>
