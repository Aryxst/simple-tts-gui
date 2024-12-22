import path from "path-browserify";
import { appDataDir } from "@tauri-apps/api/path";
import { Command } from "@tauri-apps/plugin-shell";
import { invoke } from "@tauri-apps/api/core";

export interface Root {
 _id: string;
 id: string;
 author: string;
 gated: boolean;
 inference: string;
 lastModified: string;
 likes: number;
 trendingScore: number;
 private: boolean;
 sha: string;
 downloads: number;
 tags: Array<string>;
 createdAt: string;
 modelId: string;
 siblings: Array<{
  rfilename: string;
 }>;
}

const HUGGINGFACE_TRAINED_MODELS_URL =
 "https://huggingface.co/rhasspy/piper-voices/resolve/main";
const TRAINED_MODELS_API_ENDPOINT_URL =
 "https://huggingface.co/api/models?search=piper-voices&author=rhasspy&limit=1&full=true";

export const MODELS_PATH = path.join(await appDataDir(), "models");

/**
 * Downloads a model from Hugging Face to the local machine.
 * @param model - The truncated model Huggingface URL to download.
 * @example
 * ```typescript
 * await downloadModel("en/en_GB/alan/low/en_GB-alan-low")
 * ```
 */
export async function downloadModel(model: string) {
 const splatPath = model.split("/");
 const modelName = splatPath.pop();
 const modelFolderPath = path.join(MODELS_PATH, ...splatPath);

 for (const extension of ["onnx", "onnx.json"]) {
  const command = Command.create("curl", [
   "--remote-name",
   "--remote-header-name",
   "--output-dir",
   modelFolderPath,
   "--create-dirs",
   "-L",
   `${HUGGINGFACE_TRAINED_MODELS_URL}/${model}.${extension}`,
  ]);

  console.log(`Downloaded ${modelName}.${extension}`);

  await command.execute();
 }
}

/**
 * Retrieves a list of locally stored models.
 */
export async function getLocalModels() {
 const files: Array<{ path: string; isFile: boolean; size: number }> = await invoke(
  "list_files",
  { path: MODELS_PATH }
 );
 return files;
}

/**
 * Fetches the list of remote models from Hugging Face API.
 * @returns Information about a model repository
 */
export async function getRemoteModels(): Promise<Root> {
 const response = await fetch(TRAINED_MODELS_API_ENDPOINT_URL);
 const data = await response.json();
 return data[0];
}

export type ModelByConfig = [string, [string, string]];

/**
 * Creates array of [`MODEL_NAME`, [`MODEL`, `MODEL_CONFIG_FILE`]]
 * @param array - Array of truncated Huggingface URLs or file paths.
 * @returns A grouped array of models by configuration.
 */
export function groupModelsByConfig(array: Array<string>): Array<ModelByConfig> {
 const groupedFiles = array.reduce((map, file) => {
  // Remove `.onnx` and `.onnx.json` extensions to get the base name
  const baseName = file.replace(/\.onnx(\.json)?$/, "");

  // Get the existing group for this base name or initialize a new one
  const group = map.get(baseName) || [];

  // Add the current file to the group
  group.push(file);

  // Update the map with the group
  return map.set(baseName, group);
 }, new Map());

 // Convert the map to an array and filter to include only complete pairs
 return Array.from(groupedFiles).filter(([, group]) => group.length === 2);
}
