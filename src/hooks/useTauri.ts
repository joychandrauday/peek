import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function useTauriCommand<T>(
  command: string,
  args?: Record<string, unknown>
) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        const result = await invoke<T>(command, args);
        setData(result);
      } catch (err) {
        setError(err as string);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [command, args]);

  return { data, loading, error };
}
