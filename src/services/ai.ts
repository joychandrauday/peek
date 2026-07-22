import { invoke } from "@tauri-apps/api/core";

export interface AIConfig {
  apiKey: string;
  model: string;
  maxTokens: number;
}

export async function queryAI(query: string, config: AIConfig): Promise<string> {
  try {
    const result = await invoke<string>("query_ai", {
      query,
      config: {
        api_key: config.apiKey,
        model: config.model,
        max_tokens: config.maxTokens,
      },
    });
    return result;
  } catch (error) {
    console.error("AI query failed:", error);
    throw error;
  }
}

export async function testAIConnection(config: AIConfig): Promise<string> {
  try {
    const result = await invoke<string>("test_ai_connection", {
      config: {
        api_key: config.apiKey,
        model: config.model,
        max_tokens: config.maxTokens,
      },
    });
    return result;
  } catch (error) {
    console.error("AI connection test failed:", error);
    throw error;
  }
}
