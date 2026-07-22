import { useState, useEffect } from "react";
import { testAIConnection } from "../../services/ai";

interface AISettings {
  enabled: boolean;
  apiKey: string;
  model: string;
}

function Settings() {
  const [isOpen, setIsOpen] = useState(false);
  const [settings, setSettings] = useState<AISettings>({
    enabled: false,
    apiKey: "",
    model: "openrouter/free",
  });
  const [testStatus, setTestStatus] = useState<"idle" | "testing" | "success" | "error">("idle");
  const [testMessage, setTestMessage] = useState("");

  useEffect(() => {
    const saved = localStorage.getItem("peek-ai-settings");
    if (saved) {
      setSettings(JSON.parse(saved));
    }
  }, []);

  const saveSettings = (newSettings: AISettings) => {
    setSettings(newSettings);
    localStorage.setItem("peek-ai-settings", JSON.stringify(newSettings));
  };

  const handleTestConnection = async () => {
    setTestStatus("testing");
    setTestMessage("");
    try {
      const result = await testAIConnection({
        apiKey: settings.apiKey,
        model: settings.model,
        maxTokens: 200,
      });
      setTestStatus("success");
      setTestMessage(result);
    } catch (error) {
      setTestStatus("error");
      setTestMessage(error instanceof Error ? error.message : "Connection failed");
    }
    setTimeout(() => {
      setTestStatus("idle");
      setTestMessage("");
    }, 3000);
  };

  if (!isOpen) {
    return (
      <button
        onClick={() => setIsOpen(true)}
        className="fixed bottom-4 right-4 p-2 text-gray-400 hover:text-white transition-colors"
      >
        <svg
          className="w-5 h-5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
          />
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
      </button>
    );
  }

  return (
    <div className="fixed inset-0 flex items-center justify-center z-50">
      <div
        className="absolute inset-0 bg-black/20 backdrop-blur-sm"
        onClick={() => setIsOpen(false)}
      />
      <div className="relative w-[400px] bg-gray-900 rounded-xl shadow-2xl border border-gray-700 overflow-hidden animate-fade-in">
        <div className="flex items-center justify-between px-4 py-3 border-b border-gray-700">
          <h2 className="text-white font-medium">AI Settings</h2>
          <button
            onClick={() => setIsOpen(false)}
            className="text-gray-400 hover:text-white"
          >
            <svg
              className="w-5 h-5"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        <div className="p-4 space-y-4">
          <p className="text-sm text-gray-400">
            AI is optional. PEEK works without it.
          </p>

          <div className="flex items-center justify-between">
            <label className="text-sm text-gray-300">Enable AI Fallback</label>
            <button
              onClick={() =>
                saveSettings({ ...settings, enabled: !settings.enabled })
              }
              className={`w-10 h-6 rounded-full transition-colors ${
                settings.enabled ? "bg-primary-500" : "bg-gray-700"
              }`}
            >
              <div
                className={`w-4 h-4 bg-white rounded-full transition-transform ${
                  settings.enabled ? "translate-x-5" : "translate-x-1"
                }`}
              />
            </button>
          </div>

          {settings.enabled && (
            <>
              <div>
                <label className="block text-sm text-gray-300 mb-1">
                  Provider
                </label>
                <div className="px-3 py-2 bg-gray-800 rounded text-sm text-gray-400">
                  OpenRouter
                </div>
              </div>

              <div>
                <label className="block text-sm text-gray-300 mb-1">
                  API Key
                </label>
                <input
                  type="password"
                  value={settings.apiKey}
                  onChange={(e) =>
                    saveSettings({ ...settings, apiKey: e.target.value })
                  }
                  placeholder="sk-or-v1-..."
                  className="w-full px-3 py-2 bg-gray-800 rounded text-sm text-white placeholder-gray-500 outline-none focus:ring-2 focus:ring-primary-500"
                />
              </div>

              <div>
                <label className="block text-sm text-gray-300 mb-1">
                  Model
                </label>
                <input
                  type="text"
                  value={settings.model}
                  onChange={(e) =>
                    saveSettings({ ...settings, model: e.target.value })
                  }
                  className="w-full px-3 py-2 bg-gray-800 rounded text-sm text-white outline-none focus:ring-2 focus:ring-primary-500"
                />
              </div>

              <p className="text-xs text-gray-500">
                Get free key at{" "}
                <a
                  href="https://openrouter.ai/keys"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-primary-400 hover:text-primary-300"
                >
                  openrouter.ai/keys
                </a>
              </p>

              <button
                onClick={handleTestConnection}
                disabled={testStatus === "testing" || !settings.apiKey}
                className="w-full py-2 bg-gray-800 hover:bg-gray-700 rounded text-sm text-white transition-colors disabled:opacity-50"
              >
                {testStatus === "testing"
                  ? "Testing..."
                  : testStatus === "success"
                  ? "Connected!"
                  : testStatus === "error"
                  ? "Connection failed"
                  : "Test Connection"}
              </button>

              {testMessage && (
                <p className={`text-xs ${testStatus === "success" ? "text-green-400" : "text-red-400"}`}>
                  {testMessage}
                </p>
              )}
            </>
          )}
        </div>
      </div>
    </div>
  );
}

export default Settings;
