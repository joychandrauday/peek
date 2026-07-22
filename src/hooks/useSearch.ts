import { useState, useCallback, useEffect, useRef } from "react";
import { searchCommands, SearchResult, AIConfig } from "../services/search";

interface AISettings {
  enabled: boolean;
  apiKey: string;
  model: string;
}

function getAIConfig(): AIConfig | undefined {
  const saved = localStorage.getItem("peek-ai-settings");
  if (!saved) return undefined;

  try {
    const settings: AISettings = JSON.parse(saved);
    if (!settings.enabled || !settings.apiKey) return undefined;

    return {
      api_key: settings.apiKey,
      model: settings.model || "openrouter/free",
      max_tokens: 200,
    };
  } catch {
    return undefined;
  }
}

export function useSearch() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [isLoading, setIsLoading] = useState(false);
  const debounceRef = useRef<number>();

  const performSearch = useCallback(async (searchQuery: string) => {
    if (!searchQuery.trim()) {
      setResults([]);
      return;
    }

    setIsLoading(true);
    try {
      const aiConfig = getAIConfig();
      const searchResults = await searchCommands(searchQuery, aiConfig);
      setResults(searchResults);
      setSelectedIndex(0);
    } catch (error) {
      console.error("Search error:", error);
      setResults([]);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    debounceRef.current = window.setTimeout(() => {
      performSearch(query);
    }, 150);

    return () => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
    };
  }, [query, performSearch]);

  const clearSearch = useCallback(() => {
    setQuery("");
    setResults([]);
    setSelectedIndex(0);
  }, []);

  return {
    query,
    setQuery,
    results,
    selectedIndex,
    setSelectedIndex,
    isLoading,
    clearSearch,
  };
}
