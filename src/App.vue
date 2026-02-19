<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

const rustResult = ref('')

async function handleAdd() {
  try {
    const result = await invoke('cpp_add', { a: 5, b: 3 })
    rustResult.value += '\n' + result
  } catch (error) {
    console.error('Error calling Rust add function:', error)
  }
}
async function handleProcessStr() {
  try {
    const result = await invoke('cpp_process_string', { input: 'Hello from Vue!' })
    rustResult.value += '\n' + result
  } catch (error) {
    console.error('Error calling Rust process_string function:', error)
  }
}
async function handleGetData() {
  try {
    const result = await invoke('test', { input: 'Hello from Vue!' })
    rustResult.value += '\n' + result
  } catch (error) {
    console.error('Error calling Rust get_data function:', error)
  }
}
</script>

<template>
  <h1>You did it!</h1>
  <p>
    Visit <a href="https://vuejs.org/" target="_blank" rel="noopener">vuejs.org</a> to read the
    documentation
  </p>

  <button @click="handleAdd">Call Rust Add Function</button>
  <button @click="handleProcessStr">Call Rust Process String Function</button>
  <button @click="handleGetData">Call Rust Get Data Function</button>
  <p>Result from Rust: {{ rustResult }}</p>
</template>

<style scoped></style>
