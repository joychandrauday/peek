import { invoke } from "@tauri-apps/api/core";

export interface SearchResult {
  id: number;
  title: string;
  answer: string;
  category: string;
  tags: string | null;
  score: number;
}

export interface AIConfig {
  api_key: string;
  model: string;
  max_tokens: number;
}

export async function searchCommands(
  query: string,
  aiConfig?: AIConfig
): Promise<SearchResult[]> {
  try {
    const results = await invoke<SearchResult[]>("search_commands", {
      query,
      aiConfig: aiConfig || null,
    });
    return results;
  } catch (error) {
    console.error("Search failed:", error);
    return [];
  }
}

export async function getAllCommands() {
  try {
    return await invoke("get_all_commands");
  } catch (error) {
    console.error("Failed to get commands:", error);
    return [];
  }
}

export async function addCommand(
  title: string,
  answer: string,
  category: string,
  tags?: string
) {
  try {
    return await invoke("add_command", { title, answer, category, tags });
  } catch (error) {
    console.error("Failed to add command:", error);
    throw error;
  }
}

export async function deleteCommand(id: number) {
  try {
    await invoke("delete_command", { id });
  } catch (error) {
    console.error("Failed to delete command:", error);
    throw error;
  }
}
