import { invoke } from "@tauri-apps/api/core";
import { readDir, readTextFile } from "@tauri-apps/plugin-fs";
import { join } from "@tauri-apps/api/path";

interface Command {
  title: string;
  answer: string;
  category: string;
  tags?: string;
}

async function importFromFile(filePath: string): Promise<number> {
  try {
    const content = await readTextFile(filePath);
    const commands: Command[] = JSON.parse(content);
    
    let imported = 0;
    for (const cmd of commands) {
      try {
        await invoke("add_command", {
          title: cmd.title,
          answer: cmd.answer,
          category: cmd.category,
          tags: cmd.tags || null,
        });
        imported++;
      } catch (error) {
        console.error(`Failed to import: ${cmd.title}`, error);
      }
    }
    
    return imported;
  } catch (error) {
    console.error(`Failed to read file: ${filePath}`, error);
    return 0;
  }
}

async function main() {
  const args = process.argv.slice(2);
  const dataDir = args[0] || "data";
  
  console.log(`Importing from ${dataDir}...`);
  
  try {
    const entries = await readDir(dataDir);
    let totalImported = 0;
    
    for (const entry of entries) {
      if (entry.name?.endsWith(".json")) {
        const filePath = await join(dataDir, entry.name);
        const imported = await importFromFile(filePath);
        console.log(`  ${entry.name}: ${imported} commands`);
        totalImported += imported;
      }
    }
    
    console.log(`\nTotal imported: ${totalImported} commands`);
  } catch (error) {
    console.error("Import failed:", error);
    process.exit(1);
  }
}

main();
