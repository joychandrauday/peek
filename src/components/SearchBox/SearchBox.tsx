import { useEffect, useRef } from "react";
import { SearchResult } from "../../services/search";

interface SearchBoxProps {
  query: string;
  setQuery: (query: string) => void;
  results: SearchResult[];
  selectedIndex: number;
  setSelectedIndex: (index: number) => void;
  onClose: () => void;
  clearSearch: () => void;
}

function SearchBox({
  query,
  setQuery,
  results,
  selectedIndex,
  setSelectedIndex,
  onClose,
  clearSearch,
}: SearchBoxProps) {
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        setSelectedIndex(Math.min(selectedIndex + 1, results.length - 1));
        break;
      case "ArrowUp":
        e.preventDefault();
        setSelectedIndex(Math.max(selectedIndex - 1, 0));
        break;
      case "Enter":
        if (results[selectedIndex]) {
          navigator.clipboard.writeText(results[selectedIndex].answer);
          onClose();
          clearSearch();
        }
        break;
      case "Escape":
        onClose();
        clearSearch();
        break;
    }
  };

  return (
    <div className="flex items-center px-4 py-3 border-b border-gray-700">
      <svg
        className="w-5 h-5 text-gray-400 mr-3"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
        />
      </svg>
      <input
        ref={inputRef}
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder="Ask PEEK..."
        className="flex-1 bg-transparent text-white text-lg placeholder-gray-500 outline-none"
      />
      <kbd className="hidden sm:inline-block px-2 py-1 text-xs font-mono text-gray-400 bg-gray-800 rounded">
        ESC
      </kbd>
    </div>
  );
}

export default SearchBox;
