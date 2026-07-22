import { useEffect, useCallback } from "react";
import SearchBox from "../SearchBox/SearchBox";
import AnswerCard from "../AnswerCard/AnswerCard";
import { useSearch } from "../../hooks/useSearch";

interface OverlayProps {
  isOpen: boolean;
  onClose: () => void;
}

function Overlay({ isOpen, onClose }: OverlayProps) {
  const { query, setQuery, results, selectedIndex, setSelectedIndex, clearSearch } =
    useSearch();

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onClose();
        clearSearch();
      }
    },
    [onClose, clearSearch]
  );

  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [handleKeyDown]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 flex items-center justify-center z-50">
      <div
        className="absolute inset-0 bg-black/20 backdrop-blur-sm"
        onClick={onClose}
      />
      <div className="relative w-[500px] max-h-[400px] bg-gray-900 rounded-xl shadow-2xl border border-gray-700 overflow-hidden animate-fade-in">
        <SearchBox
          query={query}
          setQuery={setQuery}
          results={results}
          selectedIndex={selectedIndex}
          setSelectedIndex={setSelectedIndex}
          onClose={onClose}
          clearSearch={clearSearch}
        />
        {results.length > 0 && (
          <div className="max-h-[280px] overflow-y-auto">
            {results.map((result, index) => (
              <AnswerCard
                key={result.id}
                result={result}
                isSelected={index === selectedIndex}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default Overlay;
