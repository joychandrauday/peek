import { useState } from "react";
import { SearchResult } from "../../services/search";

interface AnswerCardProps {
  result: SearchResult;
  isSelected: boolean;
}

function AnswerCard({ result, isSelected }: AnswerCardProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(result.answer);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  const getCategoryColor = (category: string) => {
    const colors: Record<string, string> = {
      git: "bg-orange-500/20 text-orange-400",
      docker: "bg-blue-500/20 text-blue-400",
      linux: "bg-yellow-500/20 text-yellow-400",
      javascript: "bg-yellow-400/20 text-yellow-300",
      react: "bg-cyan-500/20 text-cyan-400",
      sql: "bg-purple-500/20 text-purple-400",
      ai: "bg-green-500/20 text-green-400",
    };
    return colors[category] || "bg-gray-500/20 text-gray-400";
  };

  return (
    <div
      className={`px-4 py-3 border-b border-gray-800 cursor-pointer transition-colors ${
        isSelected ? "bg-gray-800" : "hover:bg-gray-800/50"
      }`}
      onClick={handleCopy}
    >
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center gap-2">
          <span
            className={`px-2 py-0.5 text-xs rounded-full ${getCategoryColor(
              result.category
            )}`}
          >
            {result.category}
          </span>
          <span className="text-sm text-gray-400">{result.title}</span>
        </div>
        <button
          onClick={(e) => {
            e.stopPropagation();
            handleCopy();
          }}
          className="text-gray-400 hover:text-white transition-colors"
        >
          {copied ? (
            <svg
              className="w-4 h-4 text-green-400 animate-bounce-once"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M5 13l4 4L19 7"
              />
            </svg>
          ) : (
            <svg
              className="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
              />
            </svg>
          )}
        </button>
      </div>
      <div className="font-mono text-sm text-green-400 bg-gray-950 rounded px-3 py-2 overflow-x-auto">
        {result.answer}
      </div>
      {result.tags && (
        <div className="mt-2 flex gap-1 flex-wrap">
          {result.tags.split(",").map((tag) => (
            <span
              key={tag}
              className="px-1.5 py-0.5 text-xs text-gray-500 bg-gray-800 rounded"
            >
              {tag.trim()}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}

export default AnswerCard;
