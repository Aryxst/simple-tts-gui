<script lang="ts">
 import { onMount } from "svelte";
 import { RefreshCw } from "lucide-svelte";
 import { path } from "@tauri-apps/api";
 import { invoke } from "@tauri-apps/api/core";
 import { revealItemInDir } from "@tauri-apps/plugin-opener";
 import type { ModelByConfig } from "../lib";
 import {
  getRemoteModels,
  getLocalModels,
  getModelsPath,
  downloadModel,
  groupModelsByConfig,
 } from "../lib";

 let selectedLocalModel = $state<string>();
 let selectedRemoteModel = $state<string>();
 let selectedLanguage = $state<string>();

 let localModels = $state<Array<ModelByConfig>>();
 let modelsSortedByLanguage = $state<Partial<Record<string, ModelByConfig[]>>>();

 let input = $state<string>();
 let channels = $state<number>(1);
 let sampleRate = $state<number>(22050);

 let isDownloading = $state<boolean>(false);

 onMount(async () => {
  modelsSortedByLanguage = Object.groupBy(
   groupModelsByConfig(
    (await getRemoteModels()).siblings
     .map(entry => entry.rfilename)
     .filter(entry => entry.endsWith(".onnx") || entry.endsWith(".onnx.json"))
   ),
   ([name]) => {
    return name.split("/").shift()!;
   }
  );
  await updateLocalModels();
 });
 async function updateLocalModels() {
  const modelsPath = await getModelsPath();
  localModels = groupModelsByConfig(
   (await getLocalModels()).map(entry =>
    entry.path.replace(modelsPath, "").replaceAll("\\", "/").slice(1)
   )
  );
 }
</script>

<main>
 <form
  onsubmit={async () => {
   await invoke("synthesize", {
    configPath: await path.join(await getModelsPath(), selectedLocalModel!),
    input,
    channels,
    sampleRate,
   });
  }}
 >
  <input placeholder="Synthesize" bind:value={input} required />
  <button type="submit" disabled={!(selectedLocalModel && input)}>Submit</button>
  Sample Rate
  <select bind:value={sampleRate}
   >{#each [8000, 11025, 16000, 22050, 44100, 48000, 88200, 96000, 176400, 192000, 352800, 384000] as rate}
    <option value={rate}>{rate}</option>{/each}
  </select>
  Audio channels
  <select bind:value={channels}
   >{#each [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] as rate}
    <option value={rate}>{rate}</option>{/each}
  </select>
 </form>

 <div>
  <p>Local Models:</p>
  <select bind:value={selectedLocalModel} required>
   <option selected disabled value="">Select a model</option>
   {#if localModels}
    {#each localModels as [name, [_, config]]}
     <option value={config}>
      {name}
     </option>
    {/each}
   {/if}
  </select>
  <button
   onclick={async () => {
    await updateLocalModels();
   }}><RefreshCw size={12} /></button
  >
  <button
   onclick={async () => {
    await revealItemInDir(await getModelsPath());
   }}>Open Models Directory</button
  >
 </div>

 <div>
  <p>Remote Models:</p>
  <select bind:value={selectedRemoteModel}>
   <option selected disabled value="">Select a model to download</option>
   {#if modelsSortedByLanguage && selectedLanguage}
    {@const set = new Set(localModels?.map(([name]) => name))}
    {#each modelsSortedByLanguage[selectedLanguage] as unknown as Array<ModelByConfig> as [name]}
     {@const isDisabled = set.has(name)}
     <option value={name} disabled={isDisabled}>
      {name}
      {isDisabled && "(Already downloaded)"}
     </option>
    {/each}
   {/if}
  </select>
  <select bind:value={selectedLanguage}>
   <option selected disabled value="">Select a language</option>
   {#if modelsSortedByLanguage}
    {#each Object.keys(modelsSortedByLanguage) as language}
     <option value={language}>
      {language}({modelsSortedByLanguage[language]?.length})
     </option>
    {/each}
   {/if}
  </select>
  <button
   onclick={async () => {
    isDownloading = true;
    await downloadModel(selectedRemoteModel!);
    isDownloading = false;
    await updateLocalModels();
   }}
   disabled={!!!selectedRemoteModel || isDownloading}
   >{isDownloading ? "Downloading..." : "Download"}</button
  >
 </div>
</main>
