<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";

import { ref } from "vue";
import { LogicalSize, getCurrent } from "@tauri-apps/api/window";
import { listen } from '@tauri-apps/api/event'

const curr_zoom = ref(1.0);

async function resize(zoom:number) {
  const total_height = window.outerHeight;
  const total_width = window.outerWidth;

  const vertical_padding   = 25;
  const horizontal_padding = 0;
  
  // calculate new app size
  // const new_app_height =  zoom > 1 ? (total_height - 16) * zoom + 16 : (total_height + 16) * zoom - 16;
  const new_app_height = (total_height - vertical_padding) * zoom + vertical_padding;
  const new_app_width  = total_width  * zoom + horizontal_padding;
  
  // create new logical size
  const new_size = new LogicalSize(new_app_width, new_app_height);
  
  // set size
  await getCurrent().setSize(new_size);
  
  // scale contents through rust
  const new_zoom = curr_zoom.value * zoom;
  // document.body.style.scale = new_zoom + "%";
  curr_zoom.value = new_zoom;
}

listen('zoom_in', (_) => {
  resize(0.625);
});

listen('zoom_out', (_) => {
  resize(1.6);
});
</script>

<template>
  <div class="container" :style="{zoom: curr_zoom}" @keypress="console.log">
    <!-- <h1>Welcome to Tauri!</h1>

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>

    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <p>
      Recommended IDE setup:
      <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
      +
      <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
      +
      <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
        >Tauri</a
      >
      +
      <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"
        >rust-analyzer</a
      >
    </p> -->

    <Greet />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
