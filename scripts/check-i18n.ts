import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const SKIP_FILE_REGEX = /\.test\.(ts|tsx|js|jsx)$/

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const knownKeys: string[] = [
  'error.unknown',
  'route.login',
  'route.register',
  'route.dashboard'
]
const knownDynamicKeys: Record<string, string[]> = {
  'threatLevel.': ['low', 'medium', 'high', 'critical']
}

export const srcDir = path.resolve(__dirname, '../src')
export const localesDir = path.resolve(__dirname, '../src/i18n/locales')

const staticKeyRegex = /(?:\b[tT]|i18n\.t)\s*\(\s*['"]([^'"]+)['"]\s*\)/g
const templateKeyRegex =
  /(?:\b[tT]|i18n\.t)\s*\(\s*`([^`$]+)\$\{[^}]+\}.*?`\s*\)/g

function detectRawJSXText(content: string, filePath: string) {
  const rawTexts: string[] = []

  const jsxTextRegex = />([^<{][^<>{}]*?)</g
  let match: RegExpExecArray | null

  while ((match = jsxTextRegex.exec(content)) !== null) {
    const text = match[1].trim()
    // Do not take fragments that look like TypeScript type declarations:
    const looksLikeTypeScriptGeneric =
      /<[^>]+>/.test(text) || // contains <...>
      /extends\s+/.test(text) || // contains "extends "
      /implements\s+/.test(text) || // contains "implements "
      /:\s*[^;]+;/.test(text) || // contains ": type;"
      /[{};]/.test(text) || // contains special TS characters
      /^export\s/.test(text) || // starts with export
      /^import\s/.test(text) // starts with import

    if (
      text.length > 1 &&
      /[a-zA-ZÀ-ÿ]/.test(text) &&
      !text.startsWith('{') &&
      !text.includes('Promise') &&
      !/^\d+$/.test(text) &&
      !/^[(){}[\],.:;!? ]+$/.test(text) &&
      !looksLikeTypeScriptGeneric
    ) {
      rawTexts.push(text)
    }
  }

  if (rawTexts.length > 0) {
    console.warn(`⚠️ Raw text detected in ${filePath}:`, rawTexts)
  }
}

function extractKeysFromCode(dir: string): Set<string> {
  const keys = new Set<string>()

  function walk(directory: string) {
    const files = fs.readdirSync(directory)
    for (const file of files) {
      const fullPath = path.join(directory, file)
      const stat = fs.statSync(fullPath)

      if (stat.isDirectory()) {
        walk(fullPath)
      } else if (/\.(ts|tsx|js|jsx)$/.test(file)) {
        if (SKIP_FILE_REGEX && SKIP_FILE_REGEX.test(file)) {
          // Ignorer les fichiers qui matchent le regex
          continue
        }

        const content = fs.readFileSync(fullPath, 'utf-8')
        let match: RegExpExecArray | null

        while ((match = staticKeyRegex.exec(content)) !== null) {
          keys.add(match[1])
        }

        while ((match = templateKeyRegex.exec(content)) !== null) {
          const baseKey = match[1]
          const variants = knownDynamicKeys[baseKey]
          if (variants) {
            variants.forEach(variant => {
              keys.add(`${baseKey}${variant}`)
            })
          } else {
            console.warn(
              `⚠️ Dynamic key without known variants: ${baseKey} in ${fullPath}`
            )
            keys.add(baseKey)
          }
        }

        detectRawJSXText(content, fullPath)
      }
    }
  }

  walk(dir)
  return keys
}

function loadJSON(filepath: string): any {
  try {
    return JSON.parse(fs.readFileSync(filepath, 'utf-8'))
  } catch (err) {
    console.error(`❌ Error parsing JSON: ${filepath}`)
    throw err
  }
}

function writeJSON(filepath: string, data: any) {
  fs.writeFileSync(filepath, JSON.stringify(data, null, 2) + '\n')
}

function flattenObject(obj: any, prefix = ''): Record<string, any> {
  const res: Record<string, any> = {}
  for (const key in obj) {
    const fullKey = prefix ? `${prefix}.${key}` : key
    if (typeof obj[key] === 'object' && obj[key] !== null) {
      Object.assign(res, flattenObject(obj[key], fullKey))
    } else {
      res[fullKey] = obj[key]
    }
  }
  return res
}

function buildNestedObject(keys: string[], flatData: Record<string, any>): any {
  const result: any = {}
  for (const key of keys.sort()) {
    const parts = key.split('.')
    let ref = result
    for (let i = 0; i < parts.length - 1; i++) {
      if (!ref[parts[i]]) ref[parts[i]] = {}
      ref = ref[parts[i]]
    }
    const last = parts[parts.length - 1]
    ref[last] = flatData[key]
  }
  return result
}

function addMissingKeys(flatData: Record<string, any>, keysToAdd: Set<string>) {
  for (const key of keysToAdd) {
    if (!(key in flatData)) {
      flatData[key] = null
    }
  }
}

function countNullValues(flatData: Record<string, any>): number {
  return Object.values(flatData).filter(v => v === null).length
}

function removeExtraKeys(
  flatData: Record<string, any>,
  baseKeys: Set<string>
): Record<string, any> {
  const cleaned: Record<string, any> = {}
  for (const key of Object.keys(flatData)) {
    if (baseKeys.has(key)) cleaned[key] = flatData[key]
  }
  return cleaned
}

function countEmptyValues(flatData: Record<string, any>): number {
  return Object.values(flatData).filter(v => v === '').length
}

function main() {
  try {
    const codeKeys = extractKeysFromCode(srcDir)
    const requiredKeys = new Set([...codeKeys, ...knownKeys])

    const summary: {
      lang: string
      missing: number
      nullCount: number
      emptyCount: number
    }[] = []

    const localeFiles = fs
      .readdirSync(localesDir)
      .filter(f => f.endsWith('.json'))

    let hasErrors = false

    for (const file of localeFiles) {
      const filePath = path.join(localesDir, file)
      const lang = path.basename(file, '.json')

      const data = loadJSON(filePath)
      let flatData = flattenObject(data)

      const missingKeys = [...requiredKeys].filter(k => !(k in flatData))
      addMissingKeys(flatData, new Set(missingKeys))

      flatData = removeExtraKeys(flatData, new Set(requiredKeys))

      const nullCount = countNullValues(flatData)
      const emptyCount = countEmptyValues(flatData)
      const missingCount = missingKeys.length

      if (nullCount > 0 || missingCount > 0 || emptyCount > 0) {
        hasErrors = true
      }

      const sortedData = buildNestedObject([...Object.keys(flatData)], flatData)
      writeJSON(filePath, sortedData)

      summary.push({
        lang,
        missing: missingCount,
        nullCount,
        emptyCount
      })
    }

    console.log('\n=== Language summary ===')
    console.table(summary)

    if (hasErrors) {
      console.error('\n❌ i18n check failed')
      process.exit(1)
    } else {
      console.log('\n✅ i18n check passed.')
    }
  } catch (err) {
    console.error('❌ Unexpected error:', err)
    process.exit(1)
  }
}

main()

export {
  extractKeysFromCode,
  loadJSON,
  writeJSON,
  flattenObject,
  buildNestedObject,
  addMissingKeys,
  countNullValues,
  removeExtraKeys,
  countEmptyValues
}
export default main
