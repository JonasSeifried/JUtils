<script setup lang="ts">
import { open } from "@tauri-apps/api/shell"
</script>
<template >
    <div class="this">
        <h1 style="margin-bottom: 0;">JUtils</h1>
        <div><a @click="open('https://github.com/JonasSeifried')">by Jonas Seifried</a></div>
    </div>
</template>

<style scoped>
.this {
    display: flex;
    flex-direction: column;
    width: 100%;
}
</style>